extern crate core;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use crate::frontend::codegen::CodeGen;
use crate::frontend::loader::AutoScriptLoader;
use crate::vm::interp::AutoScriptVM;

mod vm;
mod frontend;

fn main() {
    let args:Vec<OsString> = env::args_os().collect();
    assert_ne!(args.len(), 1);
    let mut loader = AutoScriptLoader::new();
    let file = PathBuf::from(args.get(1).unwrap());
    loader.add_file(&file).unwrap();
    let modules = loader.unwrap();

    let mut codegen = CodeGen::new(modules);
    let modules = codegen.translate_modules();
    let main_module_name = file.file_stem().unwrap().to_str().unwrap();

    let mut vm = AutoScriptVM::new(modules);
    vm.start(main_module_name)

}
