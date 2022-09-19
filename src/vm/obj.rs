use std::any::Any;
use std::fmt::Debug;

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

pub unsafe trait ObjInner: Debug + Any{
    #[allow(unused_variables)]
    fn trace(&self, mark: &mut dyn FnMut(*mut Obj)) {}

    fn name(&self) -> &str;
}

pub struct Obj{
    inner: Box<dyn ObjInner>,
    mark: ObjMark
}

pub enum ObjMark{
    White,
    Black
}



