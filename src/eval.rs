use crate::ast::{Expr, Loc, Opcode, UnaryOpcode};

#[derive(Debug)]
pub struct TypeErr {
    msg: String,
    expect: Type,
    actual: Type,
    loc: Loc,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Char,
    Complex(String),
}
impl Type {
    fn is_int_friendly(&self) -> bool {
        match self {
            Type::Int | Type::Char => true,
            _ => false,
        }
    }

    fn is_number_friendly(&self) -> bool {
        match self {
            Type::Int | Type::Char | Type::Float => true,
            _ => false,
        }
    }
}

/// Type check
impl Expr {
    pub fn check_type(&self) -> Result<Type, TypeErr> {
        match self {
            Expr::Int(_, _) => Ok(Type::Int),
            Expr::Float(_, _) => Ok(Type::Float),
            Expr::Bool(_, _) => Ok(Type::Bool),
            Expr::Char(_, _) => Ok(Type::Char),
            Expr::Identifier(_, _) => todo!("Not supported yet"),
            Expr::Str(_, _) => Ok(Type::Str),
            Expr::UnaryOpExpr(_, _, _) => self.check_unary_op_expr_type(),
            Expr::OpExpr(_, _, _, _) => self.check_op_expr_type(),
        }
    }

    fn check_op_expr_type(&self) -> Result<Type, TypeErr> {
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
                    if right_type != Type::Int {
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: Type::Int,
                            actual: right_type,
                            loc: *loc,
                        })
                    } else if !left_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: Type::Complex("numberic".to_string()),
                            actual: right_type,
                            loc: *loc,
                        })
                    } else {
                        Ok(left_type)
                    }
                }
                Opcode::Add => {
                    if left_type == Type::Str || right_type == Type::Str {
                        Ok(Type::Char)
                    } else if left_type == Type::Bool || right_type == Type::Bool {
                        Err(TypeErr {
                            msg: String::from("plus expr type mismatch."),
                            expect: Type::Complex("number or string".to_string()),
                            actual: Type::Bool,
                            loc: *loc,
                        })
                    } else if left_type == Type::Float || right_type == Type::Float {
                        Ok(Type::Float)
                    } else if left_type == Type::Char || right_type == Type::Char {
                        Ok(Type::Char)
                    } else if left_type == Type::Int && right_type == Type::Int {
                        Ok(Type::Int)
                    } else {
                        todo!()
                    }
                }
                Opcode::Mul => {
                    if left_type == Type::Str && right_type == Type::Int {
                        Ok(Type::Str)
                    } else if left_type == Type::Bool || right_type == Type::Bool {
                        Err(TypeErr {
                            msg: String::from("plus expr type mismatch."),
                            expect: Type::Complex("number or string".to_string()),
                            actual: Type::Bool,
                            loc: *loc,
                        })
                    } else if right_type == Type::Str {
                        Err(TypeErr {
                            msg: String::from("mul expr type mismatch."),
                            expect: Type::Complex("number".to_string()),
                            actual: Type::Bool,
                            loc: *loc,
                        })
                    } else if left_type == Type::Float || right_type == Type::Float {
                        Ok(Type::Float)
                    } else if left_type == Type::Char || right_type == Type::Char {
                        Ok(Type::Char)
                    } else if left_type == Type::Int || right_type == Type::Int {
                        Ok(Type::Int)
                    } else {
                        todo!()
                    }
                }
                Opcode::Div | Opcode::Mod | Opcode::Pow | Opcode::Sub => {
                    if !left_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: Type::Complex("numberic".to_string()),
                            actual: left_type,
                            loc: *loc,
                        })
                    } else if !right_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: Type::Complex("numberic".to_string()),
                            actual: right_type,
                            loc: *loc,
                        })
                    } else {
                        if left_type == Type::Float || right_type == Type::Float {
                            Ok(Type::Float)
                        } else if left_type == Type::Char && right_type == Type::Int {
                            Ok(Type::Char)
                        } else if left_type == Type::Int && right_type == Type::Char {
                            Ok(Type::Char)
                        } else {
                            Ok(Type::Int)
                        }
                    }
                }
                Opcode::Eq | Opcode::Lt | Opcode::Gt | Opcode::Le | Opcode::Ge => {
                    if !left_type.is_number_friendly() {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: Type::Complex("number".to_string()),
                            actual: left_type,
                            loc: *loc,
                        })
                    }else if !right_type.is_number_friendly(){
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: Type::Complex("number".to_string()),
                            actual: right_type,
                            loc: *loc,
                        })
                    }else{
                        Ok(Type::Bool)
                    }
                }
                Opcode::And | Opcode::Or => {
                    if left_type != Type::Bool {
                        Err(TypeErr {
                            msg: String::from("left expr type mismatch."),
                            expect: Type::Bool,
                            actual: left_type,
                            loc: *loc,
                        })
                    } else if right_type != Type::Bool {
                        Err(TypeErr {
                            msg: String::from("right expr type mismatch."),
                            expect: Type::Bool,
                            actual: right_type,
                            loc: *loc,
                        })
                    } else {
                        Ok(Type::Bool)
                    }
                }
                _ => todo!(),
            }
        } else {
            panic!()
        }
    }

    fn check_unary_op_expr_type(&self) -> Result<Type, TypeErr> {
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
                Type::Int | Type::Char => Ok(Type::Int),
                Type::Float => Ok(Type::Float),
                Type::Bool => Err(TypeErr {
                    msg: String::from("type mismatch."),
                    expect: Type::Complex(String::from("Numberic")),
                    actual: Type::Bool,
                    loc: *loc,
                }),
                Type::Str => Err(TypeErr {
                    msg: String::from("type mismatch."),
                    expect: Type::Complex(String::from("Numberic")),
                    actual: Type::Str,
                    loc: *loc,
                }),
                Type::Complex(_) => todo!(),
            },
            UnaryOpcode::Not => match type_expr {
                Type::Bool => Ok(Type::Bool),
                _ => Err(TypeErr {
                    msg: String::from("type mismatch."),
                    expect: Type::Bool,
                    actual: type_expr,
                    loc: *loc,
                }),
            },
        }
    }
}
