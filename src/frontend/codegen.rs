use crate::frontend::ast::ExprNode;
use crate::vm::instr::{Instr, Instructions};

pub struct CodeGen;

#[derive(Eq, PartialEq)]
enum CodeGenTy {
    Int,
    Float,
}

pub struct CodeGenInfo {
    pub instr: Instructions,
    ty: CodeGenTy,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen
    }

    pub fn translate_expr(&mut self, expr: ExprNode) -> CodeGenInfo {
        match expr {
            ExprNode::Integer(integer) => CodeGenInfo {
                instr: vec![Instr::IPush(integer)].into(),
                ty: CodeGenTy::Int,
            },
            ExprNode::Float(float) => CodeGenInfo {
                instr: vec![Instr::FPush(float)].into(),
                ty: CodeGenTy::Float,
            },
            ExprNode::Op(left, op, right) => {
                let mut left_expr = self.translate_expr(*left);
                let mut right_expr = self.translate_expr(*right);
                if left_expr.ty == CodeGenTy::Int && right_expr.ty == CodeGenTy::Int {
                    match op.as_str() {
                        "+" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IAdd].into(),
                            ty: CodeGenTy::Int,
                        },
                        "-" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::ISub].into(),
                            ty: CodeGenTy::Int,
                        },
                        "*" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IMul].into(),
                            ty: CodeGenTy::Int,
                        },
                        "/" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IDiv].into(),
                            ty: CodeGenTy::Int,
                        },
                        "%" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IRem].into(),
                            ty: CodeGenTy::Int,
                        },
                        _ => panic!()
                    }
                } else if (left_expr.ty == CodeGenTy::Int && right_expr.ty == CodeGenTy::Float)
                    || (left_expr.ty == CodeGenTy::Float && right_expr.ty == CodeGenTy::Int)
                    || (left_expr.ty == CodeGenTy::Float && right_expr.ty == CodeGenTy::Float) {
                    if left_expr.ty == CodeGenTy::Int {
                        left_expr.instr = left_expr.instr + vec![Instr::I2F].into();
                        left_expr.ty = CodeGenTy::Float;
                    } else if right_expr.ty == CodeGenTy::Int {
                        right_expr.instr = right_expr.instr + vec![Instr::I2F].into();
                        right_expr.ty = CodeGenTy::Float;
                    }
                    match op.as_str() {
                        "+" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FAdd].into(),
                            ty: CodeGenTy::Float,
                        },
                        "-" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FSub].into(),
                            ty: CodeGenTy::Float,
                        },
                        "*" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FMul].into(),
                            ty: CodeGenTy::Float,
                        },
                        "/" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FDiv].into(),
                            ty: CodeGenTy::Float,
                        },
                        "%" => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FRem].into(),
                            ty: CodeGenTy::Float,
                        },
                        _ => panic!()
                    }
                }else{
                    todo!()
                }
            }
            ExprNode::UnaryOp(op, expr) => {
                let sub_expr = self.translate_expr(*expr);
                match op.as_str() {
                    "+" => CodeGenInfo {
                        instr: vec![].into(),
                        ty: sub_expr.ty,
                    },
                    "-" => {
                        match sub_expr.ty {
                            CodeGenTy::Int => CodeGenInfo {
                                instr: sub_expr.instr + vec![Instr::INeg].into(),
                                ty: CodeGenTy::Int,
                            },
                            CodeGenTy::Float => todo!(),
                            _ => panic!()
                        }
                    }
                    _ => panic!("Unexpected operation: {}", op)
                }
            }
            _ => todo!()
        }
    }
}