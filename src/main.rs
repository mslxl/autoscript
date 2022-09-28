extern crate core;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use crate::frontend::codegen::CodeGen;

use crate::frontend::loader::AutoScriptLoader;
use crate::vm::vm::AutoScriptVM;

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

    let main_function_name = format!("V@{}.main(V", main_module_name);

    let start_time = std::time::SystemTime::now();
    vm.start(&main_function_name);
    let end_time = std::time::SystemTime::now();
    let cost_time = end_time.duration_since(start_time).unwrap().as_millis();
    println!("Cost {}ms", cost_time);
}