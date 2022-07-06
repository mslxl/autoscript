use crate::ast;
use crate::ast::FunctionDef;
use crate::symbol::Symbol;
use crate::symbol::SymbolList;
use std::rc::Rc;

fn check_type(elem: Vec<ast::ProgramElem>) -> Result<(), String> {
    let symbolList = SymbolList::new();
    for i in elem {
        match i {
            ast::ProgramElem::FuncDef(def) => {
                let name = def.name.clone();
                let sym = Symbol::FuncDef {
                    symbol_list: SymbolList::new_child(Rc::clone(&symbolList)),
                    value: def,
                };
                symbolList.borrow_mut().put(name, sym)
            }
            _ => (),
        }
    }

    panic!("TODO")
}
