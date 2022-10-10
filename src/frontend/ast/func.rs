use crate::frontend::ast::basic::TypeInfo;
use crate::vm::builtin::FunctionRustBinding;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FunctionOrigin {
    Source,
    VM,
    FFI,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionBasicInfo {
    pub name: String,
    pub module: Option<String>,
    pub param: Option<Vec<(String, TypeInfo)>>,
    pub ret: Option<TypeInfo>,
    pub origin: FunctionOrigin,
}

pub trait FunctionMatcher {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool;
}

impl FunctionMatcher for FunctionBasicInfo {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool {
        if &self.name != name {
            false // Name is not matched!!!
        } else if let Some(ref self_param) = self.param {
            if let Some(param) = param {
                if self_param.len() != param.len() {
                    // Arguments number is not matched
                    false
                } else {
                    for i in 0..self_param.len() {
                        if !param[i].is_can_convert_to(&self_param[i].1) {
                            // A param can't be converted as requirement
                            return false;
                        }
                    }
                    true // All requirement is satisfied
                }
            } else {
                // Require arguments, but got no arguments
                false
            }
        } else {
            param == None
        }
    }
}


impl FunctionBasicInfo {
    pub fn param_size(&self)->usize {
        self.param
            .map(Vec::len)
            .unwrap_or(0)
    }
    pub fn signature(&self) -> String {
        let ret = self.ret.as_ref().map(|x| x.to_string()).unwrap_or(String::from("V"));

        let name = self.name.clone();
        let module_name = self.module.as_deref().unwrap_or("");
        let param: String = match self.param {
            Some(ref params) => {
                if params.len() > 1 {
                    params.iter()
                        .map(|x| x.1.to_string())
                        .reduce(|a, b| format!("{},{}", a, b))
                        .unwrap()
                } else if !params.is_empty() {
                    params.first().unwrap().1.to_string()
                } else {
                    String::from("V")
                }
            }
            None => String::from("V")
        };

        let origin_flag = match self.origin {
            FunctionOrigin::Source => "",
            FunctionOrigin::VM => "~",
            FunctionOrigin::FFI => "#"
        };

        format!("{}{}@{}.{}({}", origin_flag, ret, module_name, name, param)
    }
}

impl ToString for FunctionBasicInfo {
    fn to_string(&self) -> String {
        self.signature()
    }
}

