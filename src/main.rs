extern crate core;

use std::path::PathBuf;

use clap::Parser;

use crate::frontend::codegen::CodeGen;
use crate::frontend::loader::ScriptFileLoader;
use crate::vm::builtin::VMBuiltinRegister;
use crate::vm::vm::AutoScriptVM;

mod vm;
mod frontend;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct VmArgs{

    /// Script file to execute
    pub file: String,

    #[arg(short, long, default_value_t = false)]
    /// Print instructions executed
    pub instr: bool
}

fn main() {
    let vm_args = VmArgs::parse();

    let mut loader = ScriptFileLoader::new();
    let file = PathBuf::from(vm_args.file.as_str());

    // load all file into modules obj
    loader.add_file(&file).unwrap();
    let mut modules = loader.unwrap();

    VMBuiltinRegister::register_prelude(&mut modules);

    let codegen = CodeGen::new(modules);
    let modules_prototype = codegen.translate_modules();
    let main_module_name = file.file_stem().unwrap().to_str().unwrap();

    let mut vm = AutoScriptVM::new(modules_prototype, vm_args);

    let main_function_name = format!("V@{}.main(V", main_module_name);

    let start_time = std::time::SystemTime::now();
    vm.start(&main_function_name);
    let end_time = std::time::SystemTime::now();
    let cost_time = end_time.duration_since(start_time).unwrap().as_millis();
    println!("Finished in {}ms", cost_time);
}