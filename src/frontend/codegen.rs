use std::rc::Rc;
use crate::frontend::ast::{ExprNode, Op, Program, StmtNode, TypeInfo, TypeRef, UnaryOp};
use crate::FunctionPrototype;
use crate::vm::instr::{Instr, Instructions};

pub struct CodeGen;


pub struct CodeGenInfo {
    pub instr: Instructions,
    ty: TypeInfo,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen
    }

    fn translate_function(&mut self, program: Program) -> FunctionPrototype {
        if let Program::Function {
            name, param, ret, block
        } = program {
            let instr = block.into_iter()
                .map(|stmt| self.translate_stmt(stmt))
                .reduce(|l,r| l + r)
                .unwrap();

            FunctionPrototype{
                name,
                local_var_size: 0,
                code: Rc::new(instr),
                ret: TypeInfo::from(ret.unwrap_or(TypeRef(String::from("unit"))).0.as_str())
            }
        }else{
            unreachable!()
        }
    }

    fn translate_stmt(&mut self, stmt: StmtNode) -> Instructions{
        match stmt {
            StmtNode::ExprStmt(expr) => self.translate_expr(expr).instr,
            StmtNode::RetStmt(expr) => match expr {
                Some(expr) => self.translate_expr(expr).instr + vec![Instr::ReturnValue].into(),
                None => vec![Instr::ReturnValue].into()
            },
        }
    }

    pub fn translate_program(&mut self, program:Program) -> FunctionPrototype{
        self.translate_function(program)
    }

    fn translate_expr(&mut self, expr: Box<ExprNode>) -> CodeGenInfo {
        match *expr {
            ExprNode::Integer(integer) => CodeGenInfo {
                instr: vec![Instr::IPush(integer)].into(),
                ty: TypeInfo::Int,
            },
            ExprNode::Float(float) => CodeGenInfo {
                instr: vec![Instr::FPush(float)].into(),
                ty: TypeInfo::Float,
            },
            ExprNode::Op(left, op, right) => {
                let mut left_expr = self.translate_expr(left);
                let mut right_expr = self.translate_expr(right);
                if left_expr.ty == TypeInfo::Int && right_expr.ty == TypeInfo::Int {
                    match op {
                        Op::Add => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IAdd].into(),
                            ty: TypeInfo::Int,
                        },
                        Op::Sub=> CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::ISub].into(),
                            ty: TypeInfo::Int,
                        },
                        Op::Mul => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IMul].into(),
                            ty: TypeInfo::Int,
                        },
                        Op::Div => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IDiv].into(),
                            ty: TypeInfo::Int,
                        },
                        Op::Rem => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::IRem].into(),
                            ty: TypeInfo::Int,
                        },
                        _ => panic!()
                    }
                } else if (left_expr.ty == TypeInfo::Int && right_expr.ty == TypeInfo::Float)
                    || (left_expr.ty == TypeInfo::Float && right_expr.ty == TypeInfo::Int)
                    || (left_expr.ty == TypeInfo::Float && right_expr.ty == TypeInfo::Float) {
                    if left_expr.ty == TypeInfo::Int {
                        left_expr.instr = left_expr.instr + vec![Instr::I2F].into();
                        left_expr.ty = TypeInfo::Float;
                    } else if right_expr.ty == TypeInfo::Int {
                        right_expr.instr = right_expr.instr + vec![Instr::I2F].into();
                        right_expr.ty = TypeInfo::Float;
                    }
                    match op {
                        Op::Add => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FAdd].into(),
                            ty: TypeInfo::Float,
                        },
                        Op::Sub => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FSub].into(),
                            ty: TypeInfo::Float,
                        },
                        Op::Mul => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FMul].into(),
                            ty: TypeInfo::Float,
                        },
                        Op::Div => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FDiv].into(),
                            ty: TypeInfo::Float,
                        },
                        Op::Rem => CodeGenInfo {
                            instr: left_expr.instr + right_expr.instr + vec![Instr::FRem].into(),
                            ty: TypeInfo::Float,
                        },
                        _ => panic!()
                    }
                }else{
                    todo!()
                }
            }
            ExprNode::UnaryOp(op, expr) => {
                let sub_expr = self.translate_expr(expr);
                match op {
                    UnaryOp::Plus=> CodeGenInfo {
                        instr: vec![].into(),
                        ty: sub_expr.ty,
                    },
                    UnaryOp::Minus => {
                        match sub_expr.ty {
                            TypeInfo::Int => CodeGenInfo {
                                instr: sub_expr.instr + vec![Instr::INeg].into(),
                                ty: TypeInfo::Int,
                            },
                            TypeInfo::Float => todo!(),
                            _ => panic!()
                        }
                    }
                    _ => panic!("Unexpected operation: {:?}", op)
                }
            }
            _ => todo!()
        }
    }
}