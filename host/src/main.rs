use wasmtime::{Engine, Module};

use crate::program::Program;

mod program;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let engine = Engine::default();
    let module = Module::from_file(&engine, file).unwrap();

    let mut program = Program::new(&engine, &module).unwrap();
    let output = program.apply("there").unwrap();
    println!("{}", output);
}
