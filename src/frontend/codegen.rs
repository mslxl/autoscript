use std::collections::HashMap;
use std::rc::Rc;

use nom::Parser;

use crate::frontend::ast::basic::{AstExprNode, AstStmtNode, Op, TypeInfo, UnaryOp};
use crate::frontend::ast::element::AstProgramFunctionImplElement;
use crate::frontend::ast::func::{FunctionBasicInfo, FunctionOrigin};
use crate::frontend::gen_info::{Env, GenInfo, VarInfo};
use crate::frontend::module_man::ProgramModuleDecl;
use crate::vm::builtin::builtin_class::ObjStr;
use crate::vm::builtin::ProgramVmFnElement;
use crate::vm::instr::{AutoScriptInstrFunction, Instr, Instructions};
use crate::vm::instr_reader::{AutoScriptInstrReader, InstrReader};
use crate::vm::mem::Obj;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;
use crate::vm::vm::{AutoScriptFunction, AutoScriptFunctionEvaluator, AutoScriptPrototype};

use super::gen_info::ConstantPoolBuilder;

pub struct CodeGen {
    env: Env,
    modules: HashMap<String, ProgramModuleDecl>,
    const_pool_builder: ConstantPoolBuilder,
}


impl CodeGen {
    pub fn new(modules: HashMap<String, ProgramModuleDecl>) -> Self {
        Self {
            env: Env::default(),
            modules,
            const_pool_builder: ConstantPoolBuilder::new(),
        }
    }

    fn find_function(
        &self,
        name: &str,
        cur_module: &str,
        access_module: Option<&str>,
        param: Option<&Vec<TypeInfo>>,
    ) -> Option<&FunctionBasicInfo> {
        let module_name = access_module.unwrap_or(cur_module);
        let module = self.modules.get(module_name).unwrap();
        let prelude = self.modules.get("prelude").unwrap();
        if let Some(header) = module.search_function(name, param) {
            return Some(header);
        } else if let Some(header) = prelude.search_function(name, param) {
            return Some(header);
        } else {
            None
        }
    }

    fn translate_function(
        &mut self,
        program: &AstProgramFunctionImplElement,
        cur_module: &str,
    ) -> AutoScriptFunction {
        self.env.push_scope();
        let arg_num = if let Some(ref param) = program.header.param {
            for i in param {
                let slot_id = self.env.current_val_size();
                self.env.val_insert(
                    i.0.clone(),
                    VarInfo {
                        ty: i.1.clone(),
                        binding_slot: slot_id,
                        is_mut: false,
                    },
                );
            }
            param.len()
        } else {
            0
        };
        let instr: Instructions = self.translate_block(&program.block, cur_module, &program.header);
        let table_size = self.env.max_val_table_size;
        self.env.pop_scope();
        AutoScriptFunction {
            name: program.header.name.clone(),
            signature: program.header.signature(),
            local_var_size: table_size,
            arg_num,
            code: Rc::new(AutoScriptInstrFunction::new(Rc::new(instr))),
        }
    }

    fn translate_stmt(
        &mut self,
        stmt: &AstStmtNode,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> Instructions {
        match stmt {
            AstStmtNode::ExprStmt(expr) => {
                let expr = self.translate_expr(expr, cur_module, header);
                let clean_instr = if expr.ty != TypeInfo::Unit { vec![Instr::Pop].into() } else { vec![].into() };
                expr.instr + clean_instr
            }
            AstStmtNode::RetStmt(expr) => match expr {
                Some(expr) => {
                    let expr_info = self.translate_expr(expr, cur_module, header);
                    assert_eq!(
                        &expr_info.ty,
                        header.ret.as_ref().unwrap_or(&TypeInfo::Unit)
                    );

                    expr_info.instr + vec![Instr::ReturnValue].into()
                }
                None => {
                    assert_eq!(
                        &TypeInfo::Unit,
                        header.ret.as_ref().unwrap_or(&TypeInfo::Unit)
                    );
                    vec![Instr::Return].into()
                }
            },
            AstStmtNode::VarStmt(name, ty_expect, is_not_mut, expr) => {
                let expr_ret = self.translate_expr(expr, cur_module, header);
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
            AstStmtNode::WhileStmt(cond, block) => {
                let cond = self.translate_expr(cond, cur_module, header);
                assert_eq!(cond.ty, TypeInfo::Bool);

                let instr = self.translate_block(block, cur_module, header);
                let unsatisfied_offset = instr.len() as i32;
                let rejudge_offset = -(unsatisfied_offset + 1 + cond.instr.len() as i32);
                cond.instr
                    + vec![Instr::JumpIfN(unsatisfied_offset + 1)].into()
                    + instr
                    + vec![Instr::Jump(rejudge_offset - 1)].into()
            }
        }
    }

    fn translate_block(
        &mut self,
        block: &[AstStmtNode],
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> Instructions {
        self.env.push_scope();
        let instr = if block.is_empty() {
            Instructions::new()
        } else if block.len() == 1 {
            self.translate_stmt(block.first().unwrap(), cur_module, header)
        } else {
            block
                .iter()
                .map(|x| self.translate_stmt(x, cur_module, header))
                .reduce(|a, b| a + b)
                .unwrap()
        };
        self.env.pop_scope();
        instr
    }

    fn try_convert_type(&self, from: &TypeInfo, target: &TypeInfo) -> Instructions {
        if from == target || target == &TypeInfo::Any {
            vec![].into()
        } else if from == &TypeInfo::Int && target == &TypeInfo::Float {
            vec![Instr::I2F].into()
        } else {
            panic!()
        }
    }


    fn convert_internal_function(&self, func: ProgramVmFnElement) -> AutoScriptFunction {
        let param_size = func.header.param_size();
        AutoScriptFunction {
            name: func.header.name.clone(),
            signature: func.header.signature(),
            local_var_size: param_size,
            arg_num: param_size,
            code: func.block,
        }
    }

    fn translate_module(&mut self, name: &str, output: &mut AutoScriptPrototype) -> Result<(), ()> {
        let src_module = self.modules.get(name).unwrap().clone();
        for element in src_module.function {
            for func in element.1 {
                let prototype = self.translate_function(&func, name);
                output.insert_function_prototype(prototype.signature.clone(), prototype);
            }
        }
        for element in src_module.vm_function {
            for func in element.1 {
                output.insert_function_prototype(func.header.signature(), self.convert_internal_function(func));
            }
        }

        Ok(())
    }

    pub fn translate_modules(mut self) -> AutoScriptPrototype {
        let mut prototype = AutoScriptPrototype::new();
        for name in self
            .modules
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<String>>()
        {
            self.translate_module(name.as_str(), &mut prototype)
                .unwrap();
        }
        prototype.replace_constant_pool(self.const_pool_builder.into());
        prototype
    }

    fn translate_expr_op(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        if let AstExprNode::Op(left, op, right) = expr.as_ref() {
            let mut left_expr = self.translate_expr(left, cur_module, header);
            let mut right_expr = self.translate_expr(right, cur_module, header);
            if left_expr.ty == TypeInfo::Int && right_expr.ty == TypeInfo::Int {
                match op {
                    Op::Add => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::IAdd].into(),
                        TypeInfo::Int,
                    ),
                    Op::Sub => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::ISub].into(),
                        TypeInfo::Int,
                    ),
                    Op::Mul => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::IMul].into(),
                        TypeInfo::Int,
                    ),
                    Op::Div => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::IDiv].into(),
                        TypeInfo::Int,
                    ),
                    Op::Rem => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::IRem].into(),
                        TypeInfo::Int,
                    ),
                    Op::Ge => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpGe].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Ne => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpNe].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Gt => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpGt].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Eq => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpEq].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Lt => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpLt].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Le => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpLe].into(),
                        TypeInfo::Bool,
                    ),
                    _ => panic!(),
                }
            } else if (left_expr.ty == TypeInfo::Int && right_expr.ty == TypeInfo::Float)
                || (left_expr.ty == TypeInfo::Float && right_expr.ty == TypeInfo::Int)
                || (left_expr.ty == TypeInfo::Float && right_expr.ty == TypeInfo::Float)
            {
                left_expr.instr =
                    left_expr.instr + self.try_convert_type(&left_expr.ty, &TypeInfo::Float);
                right_expr.instr =
                    right_expr.instr + self.try_convert_type(&right_expr.ty, &TypeInfo::Float);
                match op {
                    Op::Add => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::FAdd].into(),
                        TypeInfo::Float,
                    ),
                    Op::Sub => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::FSub].into(),
                        TypeInfo::Float,
                    ),
                    Op::Mul => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::FMul].into(),
                        TypeInfo::Float,
                    ),
                    Op::Div => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::FDiv].into(),
                        TypeInfo::Float,
                    ),
                    Op::Rem => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::FRem].into(),
                        TypeInfo::Float,
                    ),
                    Op::Ge => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpGe].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Gt => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpGt].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Ne => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpNe].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Eq => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpEq].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Lt => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpLt].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Le => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::CmpLe].into(),
                        TypeInfo::Bool,
                    ),
                    _ => panic!(),
                }
            } else if left_expr.ty == TypeInfo::Bool && right_expr.ty == TypeInfo::Bool {
                match op {
                    Op::And => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::BAnd].into(),
                        TypeInfo::Bool,
                    ),
                    Op::Or => GenInfo::new(
                        left_expr.instr + right_expr.instr + vec![Instr::BOr].into(),
                        TypeInfo::Bool,
                    ),
                    _ => panic!(),
                }
            } else {
                panic!("Operator was not supported!")
            }
        } else {
            panic!()
        }
    }

    fn translate_expr_unary(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        if let AstExprNode::UnaryOp(op, expr) = expr.as_ref() {
            let sub_expr = self.translate_expr(expr, cur_module, header);
            match op {
                UnaryOp::Plus => match sub_expr.ty {
                    TypeInfo::Int | TypeInfo::Float => GenInfo::new(vec![].into(), sub_expr.ty),
                    _ => panic!(),
                },
                UnaryOp::Minus => match sub_expr.ty {
                    TypeInfo::Int => {
                        GenInfo::new(sub_expr.instr + vec![Instr::INeg].into(), TypeInfo::Int)
                    }
                    TypeInfo::Float => {
                        GenInfo::new(sub_expr.instr + vec![Instr::FNeg].into(), TypeInfo::Float)
                    }
                    _ => panic!(),
                },
                UnaryOp::Not => match sub_expr.ty {
                    TypeInfo::Bool => {
                        GenInfo::new(sub_expr.instr + vec![Instr::BNeg].into(), TypeInfo::Bool)
                    }
                    _ => panic!(),
                },
            }
        } else {
            panic!()
        }
    }

    fn translate_expr_fncall(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        if let AstExprNode::FnCall(fn_id, param) = expr.as_ref() {
            let args: Option<Vec<GenInfo>> = if let Some(exprs) = param {
                Some(
                    exprs
                        .iter()
                        .map(|e| self.translate_expr(e, cur_module, header))
                        .collect(),
                )
            } else {
                None
            };

            let types = args
                .as_ref()
                .map(|vec| vec.iter().map(|e| e.ty.clone()).collect::<Vec<TypeInfo>>());

            let fn_name = fn_id.last().unwrap();

            let access_module = fn_id[0..fn_id.len() - 1]
                .iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{}.{}", a, b));

            let fn_header = self
                .find_function(
                    fn_name,
                    cur_module,
                    access_module.as_deref(),
                    types.as_ref(),
                )
                .expect(&format!("Can't find function: {}", fn_name));
            let args = if let Some(param) = args {
                let require_types = fn_header
                    .param
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|x| &x.1)
                    .collect::<Vec<&TypeInfo>>();
                let args = param
                    .into_iter()
                    .zip(require_types)
                    .map(|(a, e)| {
                        let convert_instr = self.try_convert_type(&a.ty, e);
                        GenInfo::new(a.instr + convert_instr, e.clone())
                    })
                    .collect::<Vec<GenInfo>>();
                Some(args)
            } else {
                None
            };

            let before_instr = if let Some(instr) = args {
                instr
                    .into_iter()
                    .fold(Instructions::new(), |a, b| a + b.instr)
            } else {
                Vec::new().into()
            };

            // let call_instr = match &fn_header.origin {
            //     FunctionOrigin::Source => Instr::Call(fn_header.signature()),
            //     FunctionOrigin::FFI => Instr::Call(fn_header.signature()),
            // };
            let call_instr = Instr::Call(fn_header.signature());

            GenInfo::new(
                before_instr + vec![call_instr].into(),
                fn_header.ret.clone().unwrap_or(TypeInfo::Unit),
            )
        } else {
            panic!()
        }
    }

    fn translate_expr_ifexpr(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        if let AstExprNode::IfExpr(cond, block, else_branch) = expr.as_ref() {
            let cond_gen = self.translate_expr(cond, cur_module, header);
            assert_eq!(cond_gen.ty, TypeInfo::Bool);
            self.env.push_scope();
            let block_code = self.translate_expr(block, cur_module, header);
            self.env.pop_scope();
            self.env.push_scope();
            let else_code = else_branch
                .as_ref()
                .map(|e| self.translate_expr(e, cur_module, header))
                .unwrap_or(GenInfo::new(vec![].into(), TypeInfo::Unit));
            self.env.pop_scope();

            let instr = cond_gen.instr
                + vec![Instr::JumpIf(else_code.instr.len() as i32 + 1)].into()
                + else_code.instr
                + vec![Instr::Jump(block_code.instr.len() as i32)].into()
                + block_code.instr;
            let ty = if block_code.ty == else_code.ty {
                block_code.ty.clone()
            } else {
                TypeInfo::Unit
            };
            if ty == TypeInfo::Unit {
                GenInfo::new(instr + vec![Instr::NPush].into(), ty)
            } else {
                GenInfo::new(instr, ty)
            }
        } else {
            panic!()
        }
    }

    fn translate_expr_blockexpr(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        if let AstExprNode::BlockExpr(block) = expr.as_ref() {
            if block.is_empty() {
                GenInfo::new(vec![].into(), TypeInfo::Unit)
            } else {
                let (head, last) = block.split_at(block.len() - 1);
                let head_instr = self.translate_block(head, cur_module, header);
                let last_stmt = last.last().unwrap();
                match last_stmt {
                    AstStmtNode::ExprStmt(expr) => {
                        let last = self.translate_expr(expr, cur_module, header);
                        GenInfo::new(head_instr + last.instr, last.ty)
                    }
                    _ => {
                        let last = self.translate_stmt(last_stmt, cur_module, header);
                        GenInfo::new(head_instr + last, TypeInfo::Unit)
                    }
                }
            }
        } else {
            panic!()
        }
    }

    fn translate_expr_assign(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        if let AstExprNode::AssignExpr(id, expr) = expr.as_ref() {
            let expr = self.translate_expr(expr, cur_module, header);
            let info = self
                .env
                .val_lookup(id)
                .expect(&format!("Can't find var named"));
            assert_eq!(info.ty, expr.ty);
            GenInfo {
                instr: expr.instr + vec![Instr::Dup, Instr::Store(info.binding_slot)].into(),
                ty: expr.ty,
            }
        } else {
            panic!()
        }
    }

    fn translate_expr(
        &mut self,
        expr: &Box<AstExprNode>,
        cur_module: &str,
        header: &FunctionBasicInfo,
    ) -> GenInfo {
        match expr.as_ref() {
            AstExprNode::Integer(integer) => {
                GenInfo::new(vec![Instr::IPush(*integer)].into(), TypeInfo::Int)
            }
            AstExprNode::Float(float) => {
                GenInfo::new(vec![Instr::FPush(*float)].into(), TypeInfo::Float)
            }
            AstExprNode::Bool(boolean) => {
                GenInfo::new(vec![Instr::BPush(*boolean)].into(), TypeInfo::Bool)
            }
            AstExprNode::String(s) => {
                let const_id = if let Some(id) = self.const_pool_builder.find(s) {
                    id
                } else {
                    let strobj = ObjStr(s.clone());
                    let obj = Obj::make_boxed(Box::new(strobj));
                    let slot = Slot::Ref(obj);
                    self.const_pool_builder.insert(s, slot)
                };
                GenInfo::new(
                    vec![Instr::CPush(const_id)].into(),
                    TypeInfo::TypeSym(String::from("String")),
                )
            }
            AstExprNode::Op(_, _, _) => self.translate_expr_op(expr, cur_module, header),
            AstExprNode::UnaryOp(_, _) => self.translate_expr_unary(expr, cur_module, header),
            AstExprNode::Ident(id) => {
                assert_eq!(id.len(), 1);
                let ident_info = self.env.val_lookup(&id.first().unwrap()).unwrap();
                GenInfo::new(
                    vec![Instr::Load(ident_info.binding_slot)].into(),
                    ident_info.ty.clone(),
                )
            }
            AstExprNode::FnCall(_, _) => self.translate_expr_fncall(expr, cur_module, header),
            AstExprNode::IfExpr(_, _, _) => self.translate_expr_ifexpr(expr, cur_module, header),
            AstExprNode::BlockExpr(_) => self.translate_expr_blockexpr(expr, cur_module, header),
            AstExprNode::AssignExpr(_, _) => self.translate_expr_assign(expr, cur_module, header),
        }
    }
}
