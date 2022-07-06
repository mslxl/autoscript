use crate::ast;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Symbol {
    IntVar {
        type_name: String,
        immutable: bool,
        value: Option<i64>,
    },
    FloatVar {
        type_name: String,
        immutable: bool,
        value: Option<f64>,
    },
    StringVar {
        type_name: String,
        immutable: bool,
        value: Option<String>,
    },
    BoolVar {
        type_name: String,
        immutable: bool,
        value: Option<bool>,
    },
    FuncDef {
        symbol_list: Rc<RefCell<SymbolList>>,
        value: ast::FunctionDef,
    },
}
pub struct SymbolList {
    parent: Option<Rc<RefCell<SymbolList>>>,
    table: HashMap<String, Rc<RefCell<Symbol>>>,
}

impl SymbolList {
    fn new_in_stack() -> SymbolList {
        SymbolList {
            parent: None,
            table: HashMap::new(),
        }
    }
    pub fn new() -> Rc<RefCell<SymbolList>> {
        Rc::new(RefCell::new(SymbolList::new_in_stack()))
    }

    pub fn new_child(parent: Rc<RefCell<SymbolList>>) -> Rc<RefCell<SymbolList>> {
        Rc::new(RefCell::new(SymbolList {
            parent: Some(parent),
            ..SymbolList::new_in_stack()
        }))
    }
    pub fn put(&mut self, id: String, sym: Symbol) {
        self.table.insert(id.to_string(), Rc::new(RefCell::new(sym)));
    }

    pub fn find(&self, id: &str) -> Option<Rc<RefCell<Symbol>>> {
        if self.table.contains_key(id) {
            self.table.get(id).map(|x|Rc::clone(x))
        } else {
            match &self.parent {
                None => None,
                Some(p) => p.borrow().find(id),
            }
        }
    }
}
