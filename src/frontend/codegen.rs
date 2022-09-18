
use crate::frontend::ast::ExprNode;
use crate::vm::instr::Instructions;
struct CodeGen;

struct CodeGenInfo{
    instr: Instructions
}

impl CodeGen{
    fn new() -> Self{
        CodeGen
    }

    fn translate_expr(expr:ExprNode) -> Instructions{
        todo!()
    }
}