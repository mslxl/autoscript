use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::frontend::ast::{ExprNode, FunctionHeader, Op, ProgramSrcElement, ProgramSrcFnElement, ProgramSrcModule, StmtNode, TypeInfo, TypeRef, UnaryOp};
use crate::vm::instr::{Instr, Instructions};
use crate::vm::interp::{AutoScriptModule, AutoScriptModuleMan, FunctionPrototype};

pub struct CodeGen {
    env: Env,
    modules: HashMap<String, ProgramSrcModule>,
}

struct VarInfo {
    ty: TypeInfo,
    binding_slot: usize,
    is_mut: bool,
}

impl VarInfo {
    fn new(ty: TypeInfo, binding_slot: usize, is_mut: bool) -> Self {
        Self {
            ty,
            binding_slot,
            is_mut,
        }
    }
}

struct EnvScope {
    ty_table: HashMap<String, ()>,
    //TODO
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

struct Env {
    stack: Vec<EnvScope>,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            stack: Vec::new()
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
    }
    fn top_mut(&mut self) -> &mut EnvScope {
        self.stack.last_mut().unwrap()
    }
    fn top(&self) -> &EnvScope {
        self.stack.last().unwrap()
    }
    pub fn val_insert(&mut self, name: String, ty: VarInfo) {
        self.top_mut().val_table.insert(name, ty);
    }
    pub fn val_lookup(&mut self, name: &str) -> Option<&VarInfo> {
        for scope in self.stack.iter().rev() {
            if scope.val_table.contains_key(name) {
                return scope.val_table.get(name);
            }
        }
        None
    }
    pub fn current_val_size(&self) -> usize {
        self.top().val_table.len()
    }
}

pub struct CodeGenInfo {
    pub instr: Instructions,
    ty: TypeInfo,
}

impl CodeGen {
    pub fn new(modules: HashMap<String, ProgramSrcModule>) -> Self {
        Self {
            env: Env::default(),
            modules,
        }
    }

    fn translate_function(&mut self, program: &ProgramSrcFnElement) -> FunctionPrototype {
        self.env.push_scope();


        let instr = program.block.iter()
            .map(|stmt| self.translate_stmt(stmt))
            .reduce(|l, r| l + r)
            .unwrap();
        let table_size = self.env.current_val_size();
        self.env.pop_scope();
        FunctionPrototype {
            name: program.header.name.clone(),
            signature: program.header.signature(),
            local_var_size: table_size,
            code: Rc::new(instr),
        }
    }

    fn translate_stmt(&mut self, stmt: &StmtNode) -> Instructions {
        match stmt {
            StmtNode::ExprStmt(expr) => self.translate_expr(expr).instr + vec![Instr::Pop].into(),
            StmtNode::RetStmt(expr) => match expr {
                Some(expr) => self.translate_expr(expr).instr + vec![Instr::ReturnValue].into(),
                None => vec![Instr::Return].into()
            },
            StmtNode::VarStmt(name, ty_expect, is_not_mut, expr) => {
                let expr_ret = self.translate_expr(expr);
                let ty_expect = ty_expect.clone().map(TypeInfo::from);
                let (convert_instr, ty) = if let Some(ty_expect) = ty_expect {
                    let instr = self.try_convert_type(&expr_ret.ty, &ty_expect);
                    (instr, ty_expect.clone())
                } else {
                    (vec![].into(), expr_ret.ty.clone())
                };
                let slot_index = self.env.current_val_size();
                let var_info = VarInfo::new(ty, slot_index, !*is_not_mut);
                self.env.val_insert(name.clone(), var_info);
                expr_ret.instr + convert_instr + vec![Instr::Store(slot_index)].into()
            }
            _ => todo!()
        }
    }

    fn try_convert_type(&mut self, from: &TypeInfo, target: &TypeInfo) -> Instructions {
        if from == target {
            vec![].into()
        } else if from == &TypeInfo::Int && target == &TypeInfo::Float {
            vec![Instr::I2F].into()
        } else {
            panic!()
        }
    }


    pub fn translate_module(&mut self, name: &str) -> AutoScriptModule {
        let src_module = self.modules.get(name).unwrap().clone();
        let mut module = AutoScriptModule::new(name.to_string());
        for element in src_module.function {
            for func in element.1 {
                let prototype = self.translate_function(&func);
                module.insert_function_prototype(prototype.signature.clone(), prototype);
            }
        }
        module
    }

    pub fn translate_modules(&mut self) -> AutoScriptModuleMan {
        let keys:Vec<String> = self.modules.keys().map(|name| name.clone()).collect();

        HashMap::from_iter(keys.into_iter().map(|name| {
            let module = self.translate_module(name.as_str());
            (name, module)
        }))
    }


    fn translate_expr(&mut self, expr: &Box<ExprNode>) -> CodeGenInfo {
        match expr.as_ref() {
            ExprNode::Integer(integer) => CodeGenInfo {
                instr: vec![Instr::IPush(*integer)].into(),
                ty: TypeInfo::Int,
            },
            ExprNode::Float(float) => CodeGenInfo {
                instr: vec![Instr::FPush(*float)].into(),
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
                        Op::Sub => CodeGenInfo {
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
                    left_expr.instr = left_expr.instr + self.try_convert_type(&left_expr.ty, &TypeInfo::Float);
                    right_expr.instr = right_expr.instr + self.try_convert_type(&right_expr.ty, &TypeInfo::Float);
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
                } else {
                    todo!()
                }
            }
            ExprNode::UnaryOp(op, expr) => {
                let sub_expr = self.translate_expr(expr);
                match op {
                    UnaryOp::Plus => CodeGenInfo {
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
            ExprNode::Ident(id) => {
                let ident_info = self.env.val_lookup(&id).unwrap();
                CodeGenInfo {
                    instr: vec![Instr::Load(ident_info.binding_slot)].into(),
                    ty: ident_info.ty.clone(),
                }
            }
            _ => todo!()
        }
    }
}