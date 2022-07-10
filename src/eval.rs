use crate::ast::{Expr, Loc, Opcode, UnaryOpcode, TypeRef};

#[derive(Debug)]
pub struct TypeErr {
    msg: String,
    expect: TypeRef,
    actual: TypeRef,
    loc: Loc,
}

impl TypeRef {
    fn is_int_friendly(&self) -> bool {
        match self {
            TypeRef::Int | TypeRef::Char => true,
            _ => false,
        }
    }

    fn is_number_friendly(&self) -> bool {
        match self {
            TypeRef::Int | TypeRef::Char | TypeRef::Float => true,
            _ => false,
        }
    }
}

/// Type check
impl Expr {
    pub fn check_type(&self) -> Result<TypeRef, TypeErr> {
        match self {
            Expr::Int(_, _) => Ok(TypeRef::Int),
            Expr::Float(_, _) => Ok(TypeRef::Float),
            Expr::Bool(_, _) => Ok(TypeRef::Bool),
            Expr::Char(_, _) => Ok(TypeRef::Char),
            Expr::Identifier(_, _) => todo!("Not supported yet"),
            Expr::Str(_, _) => Ok(TypeRef::Str),
            Expr::UnaryOpExpr(_, _, _) => self.check_unary_op_expr_type(),
            Expr::OpExpr(_, _, _, _) => self.check_op_expr_type(),
            _ => todo!()
        }
    }

    fn check_op_expr_type(&self) -> Result<TypeRef, TypeErr> {
        if let Expr::OpExpr(loc, lexpr, op, rexpr) = self {
            let left_type = lexpr.check_type()?;
            let right_type = rexpr.check_type()?;
            match op {
                Opcode::Lsh
                | Opcode::Rsh
                | Opcode::RshUnsigned
                | Opcode::BitAnd
                | Opcode::BitOr
                | Opcode::BitXor => {
                    if right_type != TypeRef::Int {
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: TypeRef::Int,
                            actual: right_type,
                            loc: *loc,
                        })
                    } else if !left_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: TypeRef::Complex("numberic".to_string()),
                            actual: right_type,
                            loc: *loc,
                        })
                    } else {
                        Ok(left_type)
                    }
                }
                Opcode::Add => {
                    if left_type == TypeRef::Str || right_type == TypeRef::Str {
                        Ok(TypeRef::Char)
                    } else if left_type == TypeRef::Bool || right_type == TypeRef::Bool {
                        Err(TypeErr {
                            msg: String::from("plus expr type mismatch."),
                            expect: TypeRef::Complex("number or string".to_string()),
                            actual: TypeRef::Bool,
                            loc: *loc,
                        })
                    } else if left_type == TypeRef::Float || right_type == TypeRef::Float {
                        Ok(TypeRef::Float)
                    } else if left_type == TypeRef::Char || right_type == TypeRef::Char {
                        Ok(TypeRef::Char)
                    } else if left_type == TypeRef::Int && right_type == TypeRef::Int {
                        Ok(TypeRef::Int)
                    } else {
                        todo!()
                    }
                }
                Opcode::Mul => {
                    if left_type == TypeRef::Str && right_type == TypeRef::Int {
                        Ok(TypeRef::Str)
                    } else if left_type == TypeRef::Bool || right_type == TypeRef::Bool {
                        Err(TypeErr {
                            msg: String::from("plus expr type mismatch."),
                            expect: TypeRef::Complex("number or string".to_string()),
                            actual: TypeRef::Bool,
                            loc: *loc,
                        })
                    } else if right_type == TypeRef::Str {
                        Err(TypeErr {
                            msg: String::from("mul expr type mismatch."),
                            expect: TypeRef::Complex("number".to_string()),
                            actual: TypeRef::Str,
                            loc: *loc,
                        })
                    } else if left_type == TypeRef::Float || right_type == TypeRef::Float {
                        Ok(TypeRef::Float)
                    } else if left_type == TypeRef::Char || right_type == TypeRef::Char {
                        Ok(TypeRef::Char)
                    } else if left_type == TypeRef::Int || right_type == TypeRef::Int {
                        Ok(TypeRef::Int)
                    } else {
                        todo!()
                    }
                }
                Opcode::Div | Opcode::Mod | Opcode::Pow | Opcode::Sub => {
                    if !left_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: TypeRef::Complex("numberic".to_string()),
                            actual: left_type,
                            loc: *loc,
                        })
                    } else if !right_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: TypeRef::Complex("numberic".to_string()),
                            actual: right_type,
                            loc: *loc,
                        })
                    } else {
                        if left_type == TypeRef::Float || right_type == TypeRef::Float {
                            Ok(TypeRef::Float)
                        } else if left_type == TypeRef::Char && right_type == TypeRef::Int {
                            Ok(TypeRef::Char)
                        } else if left_type == TypeRef::Int && right_type == TypeRef::Char {
                            Ok(TypeRef::Char)
                        } else {
                            Ok(TypeRef::Int)
                        }
                    }
                }
                Opcode::Eq | Opcode::Lt | Opcode::Gt | Opcode::Le | Opcode::Ge => {
                    if !left_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: TypeRef::Complex("number".to_string()),
                            actual: left_type,
                            loc: *loc,
                        })
                    }else if !right_type.is_number_friendly(){
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: TypeRef::Complex("number".to_string()),
                            actual: right_type,
                            loc: *loc,
                        })
                    }else{
                        Ok(TypeRef::Bool)
                    }
                }
                Opcode::And | Opcode::Or => {
                    if left_type != TypeRef::Bool {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: TypeRef::Bool,
                            actual: left_type,
                            loc: *loc,
                        })
                    } else if right_type != TypeRef::Bool {
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: TypeRef::Bool,
                            actual: right_type,
                            loc: *loc,
                        })
                    } else {
                        Ok(TypeRef::Bool)
                    }
                }
                _ => todo!(),
            }
        } else {
            panic!()
        }
    }

    fn check_unary_op_expr_type(&self) -> Result<TypeRef, TypeErr> {
        let (loc, op, expr) = match self {
            Expr::UnaryOpExpr(loc, op, expr) => (loc, op, expr),
            _ => panic!(),
        };

        let type_expr = expr.check_type()?;
        match op {
            UnaryOpcode::Inc
            | UnaryOpcode::Dec
            | UnaryOpcode::BitNot
            | UnaryOpcode::Neg
            | UnaryOpcode::Pos => match type_expr {
                TypeRef::Int | TypeRef::Char => Ok(TypeRef::Int),
                TypeRef::Float => Ok(TypeRef::Float),
                TypeRef::Bool => Err(TypeErr {
                    msg: String::from("type mismatch."),
                    expect: TypeRef::Complex(String::from("Numberic")),
                    actual: TypeRef::Bool,
                    loc: *loc,
                }),
                TypeRef::Str => Err(TypeErr {
                    msg: String::from("type mismatch."),
                    expect: TypeRef::Complex(String::from("Numberic")),
                    actual: TypeRef::Str,
                    loc: *loc,
                }),
                TypeRef::Complex(_) => todo!(),
                TypeRef::Unit => Err(TypeErr{
                    msg: String::from("type mismatch"),
                    expect: TypeRef::Complex(String::from("Numberic")),
                    actual: TypeRef::Unit,
                    loc: *loc,
                })
            },
            UnaryOpcode::Not => match type_expr {
                TypeRef::Bool => Ok(TypeRef::Bool),
                _ => Err(TypeErr {
                    msg: String::from("type mismatch."),
                    expect: TypeRef::Bool,
                    actual: type_expr,
                    loc: *loc,
                }),
            },
        }
    }
}
