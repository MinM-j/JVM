use std::fs;

use parser::class_file_reader::ClassFileReader;
use vm::class_loader::class_loader::ClassLoader;
use vm::vm::VM;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        panic!("Invalid args\n usage: <exe> [ parse | run ] <class_file>");
    }

    match args.get(1).unwrap().as_str() {
        "parse" => parse(&args[2].as_str()),
        "run" => run(&args[2].as_str()).await,
        cmd => panic!("command {} not implemented yet.", cmd),
    }
}

async fn run(main_class: &str) {
    //   let mut vm = VM::new();
    // TODO provide args as string array
    // let start_args = vec![Value::Int(0)];
    //  vm.start(main_class, start_args);
    //let class = class_manager.get_or_resolve_class(main_class).unwrap();
    println!("{main_class}");
    let mut vm = VM::new();
    let _ = vm.class_loader.add_directory_entry("".to_string());
    //let _ = vm.class_loader.add_directory_entry("../Temp/java/".to_string());
    let _ = vm.class_loader.add_jar_entry(base.to_string());
    let _ = vm.invoke_main(main_class).await;

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

const base: &str = "/usr/lib/jvm/java-23-openjdk/jmods/java.base.jmod";
