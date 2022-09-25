extern crate core;

use std::env;
use std::ffi::OsString;
use crate::frontend::loader::AutoScriptLoader;
use crate::vm::interp::AutoScriptVM;

mod vm;
mod frontend;



fn main() {
    let args:Vec<OsString> = env::args_os().collect();
    assert_ne!(args.len(), 1);
    let mut loader = AutoScriptLoader::new();
    loader.add_file(args.get(1).unwrap().into()).unwrap();
    let modules = loader.unwrap();

}
