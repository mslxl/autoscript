use crate::frontend::ast::TypeInfo;
use crate::vm::const_pool::ConstantPool;
use crate::vm::instr::Instructions;
use crate::vm::slot::Slot;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

pub struct GenInfo {
    pub instr: Instructions,
    pub ty: TypeInfo,
}

impl GenInfo {
    pub fn new(instr: Instructions, ty: TypeInfo) -> Self {
        Self { instr, ty }
    }
}

pub struct VarInfo {
    pub ty: TypeInfo,
    pub binding_slot: usize,
    pub is_mut: bool,
}

impl VarInfo {
    pub fn new(ty: TypeInfo, binding_slot: usize, is_mut: bool) -> Self {
        Self {
            ty,
            binding_slot,
            is_mut,
        }
    }
}

pub struct EnvScope {
    ty_table: HashMap<String, ()>,
    val_table: HashMap<String, VarInfo>, // 值环境
}

impl Default for EnvScope {
    fn default() -> Self {
        Self {
            ty_table: HashMap::new(),
            val_table: HashMap::new(),
        }
    }
}

pub struct Env {
    pub stack: Vec<EnvScope>,
    pub max_val_table_size: usize,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            stack: Vec::new(),
            max_val_table_size: 0,
        };
        env.push_scope();
        env
    }
}

impl Env {
    pub fn push_scope(&mut self) {
        self.stack.push(EnvScope::default())
    }
    pub fn pop_scope(&mut self) {
        self.stack.pop();
        if self.stack.is_empty() {
            self.max_val_table_size = 0;
        }
    }
    fn top_mut(&mut self) -> &mut EnvScope {
        self.stack.last_mut().unwrap()
    }
    fn top(&self) -> &EnvScope {
        self.stack.last().unwrap()
    }
    /// Insert a value to value environment, return slot index
    pub fn val_insert(&mut self, name: String, ty: VarInfo) -> usize {
        self.top_mut().val_table.insert(name, ty);
        self.current_val_size() - 1
    }
    pub fn val_lookup(&mut self, name: &str) -> Option<&VarInfo> {
        for scope in self.stack.iter().rev() {
            if scope.val_table.contains_key(name) {
                return scope.val_table.get(name);
            }
        }
        None
    }
    pub fn current_val_size(&mut self) -> usize {
        let len = self
            .stack
            .iter()
            .map(|x| x.val_table.len())
            .reduce(|a, b| a + b)
            .unwrap_or(0);
        self.max_val_table_size = max(self.max_val_table_size, len);
        len
    }
}

pub struct ConstantPoolBuilder {
    pool: Vec<Slot>,
    table: HashMap<u64, usize>
}

impl ConstantPoolBuilder {
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            table: HashMap::new()
        }
    }

    fn hash_key(key: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn insert(&mut self, key: &impl Hash, slot: Slot) -> usize{
        let key = Self::hash_key(key);
        
        if self.table.contains_key(&key) {
            self.table.get(&key).unwrap().clone()
        }else{
            self.pool.push(slot);
            let pos = self.pool.len() - 1;
            self.table.insert(key, pos);
            pos
        }
    }


    pub fn find(&self, key: &impl Hash) -> Option<usize> {
        let key = Self::hash_key(key);
        self.table.get(&key).map(usize::clone)
    } 
}

impl Into<ConstantPool> for ConstantPoolBuilder {
    fn into(self) -> ConstantPool {
        self.pool.into()
    }
}