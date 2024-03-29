use std::alloc::{GlobalAlloc, System};
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, null_mut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize};
use std::sync::atomic::Ordering::SeqCst;
use std::thread::yield_now;

pub trait AsAny {
    fn any_ref(&self) -> &dyn Any;
    fn any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn any_ref(&self) -> &dyn Any {
        self
    }
    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub unsafe trait ObjCore: Debug + Any + ToString {
    #[allow(unused_variables)]
    fn trace(&self, mark: &mut dyn FnMut(*mut Obj)) {}

    fn name(&self) -> &str;
}

pub struct Obj {
    core: Box<dyn ObjCore>,
    mark: ObjMark,
    rw: AtomicU32,
    prev: *mut Obj,
}

impl ToString for Obj{
    fn to_string(&self) -> String {
        self.core.to_string()
    }
}

static PREV_ALLOC: AtomicU64 = AtomicU64::new(0);

impl Obj {
    pub fn make_boxed(core: Box<dyn ObjCore>) -> *mut Self {
        let mut obj = Box::new(Self {
            core,
            mark: ObjMark::White,
            rw: AtomicU32::new(0),
            prev: NonNull::dangling().as_ptr(),
        });
        obj.prev = PREV_ALLOC.swap(&*obj as *const _ as _, SeqCst) as _;
        Box::leak(obj)
    }

    const MUTATOR_WRITE: u32 = u32::MAX;
    const COLLECTOR_READ_MASK: u32 = !(u32::MAX >> 1);
}

#[derive(PartialEq)]
pub enum ObjMark {
    White,
    Black,
}


#[derive(Debug)]
pub struct MemInner {
    cap: usize,
}

impl MemInner {
    unsafe fn mutator_read(&self, obj: *mut Obj) -> ObjReader<'_> {
        ObjReader::new(obj)
    }

    unsafe fn mutator_write(&self, obj: *mut Obj) -> ObjWriter<'_> {
        ObjWriter::new(obj)
    }

}

struct ObjReader<'a>(*mut Obj, PhantomData<&'a ()>);

impl ObjReader<'_> {
    unsafe fn new(obj: *mut Obj) -> Self {
        while {
            let rw = (*obj).rw.load(SeqCst);
            assert_eq!(rw, Obj::MUTATOR_WRITE);
            (rw & Obj::COLLECTOR_READ_MASK != 0)
                || (*obj).rw
                .compare_exchange_weak(rw, rw + 1, SeqCst, SeqCst)
                .is_err()
        } {
            yield_now()
        }
        Self(obj, PhantomData)
    }
}


impl Drop for ObjReader<'_> {
    fn drop(&mut self) {
        let rw = unsafe {
            &*self.0
        }.rw.fetch_sub(1, SeqCst);
        assert_ne!(rw, 0);
        assert_eq!(rw & Obj::COLLECTOR_READ_MASK, 0);
    }
}

impl Deref for ObjReader<'_> {
    type Target = dyn ObjCore;
    fn deref(&self) -> &Self::Target {
        &*unsafe {
            &*self.0
        }.core
    }
}

struct ObjWriter<'a>(*mut Obj, PhantomData<&'a ()>);
impl ObjWriter<'_> {
    unsafe fn new(obj: *mut Obj) -> Self {
        while {
            let rw = (*obj).rw.load(SeqCst);
            assert_eq!(rw & !Obj::COLLECTOR_READ_MASK, 0);
            (rw & Obj::COLLECTOR_READ_MASK != 0)
                || (*obj)
                .rw
                .compare_exchange_weak(rw, Obj::MUTATOR_WRITE, SeqCst, SeqCst)
                .is_err()
        } {
            yield_now();
        }
        Self(obj, PhantomData)
    }
}
impl Drop for ObjWriter<'_> {
    fn drop(&mut self) {
        let rw = unsafe { &*self.0 }.rw.swap(0, SeqCst);
        assert_eq!(rw, Obj::MUTATOR_WRITE);
    }
}
impl Deref for ObjWriter<'_> {
    type Target = dyn ObjCore;
    fn deref(&self) -> &Self::Target {
        &*unsafe { &*self.0 }.core
    }
}
impl DerefMut for ObjWriter<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *unsafe { &mut *self.0 }.core
    }
}

struct CollectorRead(*mut Obj);
impl CollectorRead {
    unsafe fn new(obj: *mut Obj) -> Self {
        while {
            let rw = (*obj).rw.load(SeqCst);
            assert_ne!(rw, Obj::MUTATOR_WRITE);
            assert_eq!(rw & Obj::COLLECTOR_READ_MASK, 0);
            rw == Obj::MUTATOR_WRITE
                || (*obj)
                .rw
                .compare_exchange_weak(rw, rw ^ Obj::COLLECTOR_READ_MASK, SeqCst, SeqCst)
                .is_err()
        } {
            yield_now()
        }
        Self(obj)
    }
}
impl Drop for CollectorRead {
    fn drop(&mut self) {
        let rw = unsafe { &*self.0 }
            .rw
            .fetch_xor(Obj::COLLECTOR_READ_MASK, SeqCst);
        assert_ne!(rw, Obj::MUTATOR_WRITE);
        assert_ne!(rw & Obj::COLLECTOR_READ_MASK, 0);
    }
}

#[derive(Debug)]
pub struct Mem(RwLock<MemInner>);

impl Mem {
    const INITIAL_CAP: usize = 5 << 20;
    pub const COLLECT_THRESHOLD: f32 = 0.8;
    pub fn new() -> Self {
        Self(RwLock::new(MemInner {
            cap: Mem::INITIAL_CAP
        }))
    }
}


pub struct MemStat {
    pub num_alloc: AtomicU32,
    pub num_dealloc: AtomicU32,
    pub num_realloc: AtomicU32,
    pub allocated: AtomicUsize,
}

#[global_allocator]
pub static MEM_STAT: MemStat = MemStat {
    num_alloc: AtomicU32::new(0),
    num_dealloc: AtomicU32::new(0),
    num_realloc: AtomicU32::new(0),
    allocated: AtomicUsize::new(0),
};

unsafe impl GlobalAlloc for MemStat {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.num_alloc.fetch_add(1, SeqCst);
        self.allocated.fetch_add(layout.size(), SeqCst);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        self.num_dealloc.fetch_add(1, SeqCst);
        self.allocated.fetch_sub(layout.size(), SeqCst);
        System.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.num_alloc.fetch_add(1, SeqCst);
        self.allocated.fetch_add(layout.size(), SeqCst);
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        self.num_realloc.fetch_add(1, SeqCst);
        if layout.size() < new_size {
            self.allocated.fetch_add(new_size - layout.size(), SeqCst);
        } else {
            self.allocated.fetch_sub(layout.size() - new_size, SeqCst);
        }
        System.realloc(ptr, layout, new_size)
    }
}

impl MemInner {
    fn make_boxed(&self, core: Box<dyn ObjCore>) -> *mut Obj {
        Obj::make_boxed(core)
    }

    pub fn load_factor(&self) -> f32 {
        MEM_STAT.allocated.load(SeqCst) as f32 / self.cap as f32
    }

    fn update_cap(&mut self) -> usize {
        self.cap = Mem::INITIAL_CAP
            .max((MEM_STAT.allocated.load(SeqCst) as f32 / (Mem::COLLECT_THRESHOLD / 2.)) as _);
        self.cap
    }

    unsafe fn collect(&mut self, root_iter: impl Iterator<Item = *mut Obj>) -> usize {
        let mut mark_list = Vec::new();
        mark_list.extend(root_iter);
        // mark phase
        // collector can be concurrent reader to mutators, use `CollectorRead`
        // guard as read-write spinlock
        while let Some(obj) = mark_list.pop() {
            let obj = CollectorRead::new(obj);
            let obj = &mut *obj.0;
            obj.mark = ObjMark::Black;
            obj.core.trace(&mut |traced| {
                if (*traced).mark == ObjMark::White {
                    mark_list.push(traced);
                }
            });
        }

        // sweep phase, no plan to make it better than stop the world
        // must grab a global mutex at this point. In current implementation
        // the global mutex is acquired before mark phase so no need here
        // use a `ObjWrite` guard to assert global exclusive accessing
        let saved_prev = PREV_ALLOC.load(SeqCst) as *mut Obj;
        let mut scan_obj = saved_prev;
        let mut prev_obj = null_mut();
        while !scan_obj.is_null() {
            let obj_guard = ObjWriter::new(scan_obj);
            let obj = &mut *obj_guard.0;
            let next_scan = obj.prev;
            if obj.mark == ObjMark::Black {
                obj.mark = ObjMark::White;
                obj.prev = prev_obj;
                prev_obj = obj;
            } else {
                drop(obj_guard);
                drop(Box::from_raw(obj));
            }
            scan_obj = next_scan;
        }
        let res = PREV_ALLOC.compare_exchange(saved_prev as _, prev_obj as _, SeqCst, SeqCst);
        assert!(res.is_ok());

        self.update_cap()
    }
}


pub struct Mutator<'a>(RwLockReadGuard<'a, MemInner>);
pub struct Collector<'a>(RwLockWriteGuard<'a, MemInner>);

impl Mem {
    pub fn mutator(&self) -> Mutator<'_> {
        Mutator(self.0.read().unwrap())
    }

    pub fn collector(&self) -> Collector<'_> {
        Collector(self.0.write().unwrap())
    }

    pub fn load_factor(&self) -> f32 {
        self.0.read().unwrap().load_factor()
    }
}

impl Mutator<'_> {
    pub fn make(&self, core: impl ObjCore + 'static) -> *mut Obj {
        self.make_boxed(Box::new(core))
    }

    pub fn make_boxed(&self, core: Box<dyn ObjCore>) -> *mut Obj {
        self.0.make_boxed(core)
    }

    /// # Safety
    /// `obj` must be returned by `make`, and be traced, i.e. not get collected
    /// in all previous collection.
    pub unsafe fn read(&self, obj: *mut Obj) -> impl Deref<Target = dyn ObjCore> + '_ {
        self.0.mutator_read(obj)
    }

    /// # Safety
    /// Same as `read`.
    pub unsafe fn write(&self, obj: *mut Obj) -> impl DerefMut<Target = dyn ObjCore> + '_ {
        self.0.mutator_write(obj)
    }
}

impl Collector<'_> {
    /// # Safety
    /// All objects in `root_iter` must be valid and alive, i.e. the pointer
    /// must be returned by `make`, and pointed object must be either present
    /// in `root_iter`, or be traced, in all previous collection.
    ///
    /// All traced objects must implement `ObjCore::trace` correctly.
    pub unsafe fn collect(&mut self, root_iter: impl Iterator<Item = *mut Obj>) -> usize {
        self.0.collect(root_iter)
    }
}
