use std::collections::HashMap;
use std::rc::Rc;
use crate::frontend::ast::{ExprNode, Op, ProgramSrcElement, ProgramSrcFnElement, ProgramSrcModule, StmtNode, TypeInfo, UnaryOp};
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

    fn translate_function(&mut self, program: &ProgramSrcFnElement, cur_module: &str) -> FunctionPrototype {
        self.env.push_scope();
        let arg_num = if let Some(ref param) = program.header.param {
            for i in param {
                let slot_id = self.env.current_val_size();
                self.env.val_insert(i.0.clone(), VarInfo {
                    ty: i.1.clone(),
                    binding_slot: slot_id,
                    is_mut: false,
                })
            }
            param.len()
        } else {
            0
        };
        let instr = program.block.iter()
            .map(|stmt| self.translate_stmt(stmt, cur_module))
            .reduce(|l, r| l + r)
            .unwrap();
        let table_size = self.env.current_val_size();
        self.env.pop_scope();
        FunctionPrototype {
            name: program.header.name.clone(),
            signature: program.header.signature(),
            local_var_size: table_size,
            arg_num,
            code: Rc::new(instr),
        }
    }

    fn translate_stmt(&mut self, stmt: &StmtNode, cur_module: &str) -> Instructions {
        match stmt {
            StmtNode::ExprStmt(expr) => self.translate_expr(expr, cur_module).instr + vec![Instr::Pop].into(),
            StmtNode::RetStmt(expr) => match expr {
                Some(expr) => self.translate_expr(expr, cur_module).instr + vec![Instr::ReturnValue].into(),
                None => vec![Instr::Return].into()
            },
            StmtNode::VarStmt(name, ty_expect, is_not_mut, expr) => {
                let expr_ret = self.translate_expr(expr, cur_module);
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

    fn try_convert_type(&self, from: &TypeInfo, target: &TypeInfo) -> Instructions {
        if from == target {
            vec![].into()
        } else if from == &TypeInfo::Int && target == &TypeInfo::Float {
            vec![Instr::I2F].into()
        } else {
            panic!()
        }
    }


    fn translate_module(&mut self, name: &str) -> AutoScriptModule {
        let src_module = self.modules.get(name).unwrap().clone();
        let mut module = AutoScriptModule::new(name.to_string());
        for element in src_module.function {
            for func in element.1 {
                let prototype = self.translate_function(&func, name);
                module.insert_function_prototype(prototype.signature.clone(), prototype);
            }
        }
        module
    }

    pub fn translate_modules(&mut self) -> AutoScriptModuleMan {
        let keys: Vec<String> = self.modules.keys().map(|name| name.clone()).collect();

        HashMap::from_iter(keys.into_iter().map(|name| {
            let module = self.translate_module(name.as_str());
            (name, module)
        }))
    }


    fn translate_expr(&mut self, expr: &Box<ExprNode>, cur_module: &str) -> CodeGenInfo {
        match expr.as_ref() {
            ExprNode::Integer(integer) => CodeGenInfo {
                instr: vec![Instr::IPush(*integer)].into(),
                ty: TypeInfo::Int,
            },
            ExprNode::Float(float) => CodeGenInfo {
                instr: vec![Instr::FPush(*float)].into(),
                ty: TypeInfo::Float,
            },
            ExprNode::Bool(boolean) => CodeGenInfo{
                instr: vec![Instr::BPush(*boolean)].into(),
                ty:TypeInfo::Bool,
            },
            ExprNode::Op(left, op, right) => {
                let mut left_expr = self.translate_expr(left, cur_module);
                let mut right_expr = self.translate_expr(right, cur_module);
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
                        Op::Ge => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpGe].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Ge => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpGe].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Gt => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpGt].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Eq => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpEq].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Lt => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpLt].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Le => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpLe].into(),
                            ty: TypeInfo::Bool
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
                        Op::Ge => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpGe].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Ge => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpGe].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Gt => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpGt].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Eq => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpEq].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Lt => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpLt].into(),
                            ty: TypeInfo::Bool
                        },
                        Op::Le => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::CmpLe].into(),
                            ty: TypeInfo::Bool
                        },
                        _ => panic!()
                    }
                } else if left_expr.ty == TypeInfo::Bool && right_expr.ty == TypeInfo::Bool {
                    match op {
                        Op::And => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::BAnd].into(),
                            ty: TypeInfo::Bool
                        } ,
                        Op::Or => CodeGenInfo{
                            instr: left_expr.instr + right_expr.instr + vec![Instr::BOr].into(),
                            ty: TypeInfo::Bool
                        } ,
                        _ =>  panic!()
                    }
                }else{
                    todo!()
                }
            }
            ExprNode::UnaryOp(op, expr) => {
                let sub_expr = self.translate_expr(expr, cur_module);
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
                            TypeInfo::Float => CodeGenInfo {
                                instr: sub_expr.instr + vec![Instr::FNeg].into(),
                                ty: TypeInfo::Float,
                            },
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
            ExprNode::FnCall(fn_name, param) => {
                let args: Option<Vec<CodeGenInfo>> = if let Some(exprs) = param {
                    Some(exprs.iter().map(|e| self.translate_expr(e, cur_module)).collect())
                } else {
                    None
                };

                let types = args.as_ref()
                    .map(|vec| vec.iter().map(|e| e.ty.clone())
                        .collect::<Vec<TypeInfo>>());
                let function_info = self.modules
                    .get(cur_module)
                    .unwrap()
                    .search_function(fn_name, types.as_ref())
                    .unwrap();
                let args = if let Some(param) = args {
                    let require_types = function_info.param
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|x| &x.1)
                        .collect::<Vec<&TypeInfo>>();
                    let args = param.into_iter()
                        .zip(require_types)
                        .map(|(a,e)| {
                            let convert_instr = self.try_convert_type(&a.ty, e);
                            CodeGenInfo{
                                instr:a.instr + convert_instr,
                                ty: e.clone()
                            }
                        })
                        .collect::<Vec<CodeGenInfo>>();
                    Some(args)
                }else{
                    None
                };

                let before_instr = if let Some(instr) = args {
                    instr.into_iter().fold(Instructions::new(), |a, b| a + b.instr)
                } else {
                    Vec::new().into()
                };

                CodeGenInfo {
                    instr: before_instr + vec![Instr::Call(function_info.signature())].into(),
                    ty: function_info.ret.clone().unwrap_or(TypeInfo::Unit),
                }
            }
            _ => todo!()
        }
    }
}