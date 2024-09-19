use std::fs;

use parser::class_file_reader::ClassFileReader;
use vm::vm::Value;
use vm::vm::VM;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        panic!("Invalid args\n usage: <exe> [ parse | run ] <class_file>");
    }

    match args.get(1).unwrap().as_str() {
        "parse" => parse(&args[2].as_str()),
        "run" => run(&args[2].as_str()),
        cmd => panic!("command {} not implemented yet.", cmd),
    }
}

fn run(main_class: &str) {
    let mut vm = VM::new();
    // TODO provide args as string array
    let start_args = vec![Value::Int(0)];
    vm.start(main_class, start_args);
    //let class = class_manager.get_or_resolve_class(main_class).unwrap();
    println!("run {main_class}");
    //dbg!(class);
}

fn parse(class: &str) {
    println!("parsing class file {}", class);

    //TODO: add size bound
    //read whole file to memory as Vec<u8>
    let content = fs::read(class).unwrap();

    let reader = ClassFileReader::new(content);
    let class_file = reader.parse();
    dbg!(class_file.unwrap());

    println!("Parsing completed");
}
