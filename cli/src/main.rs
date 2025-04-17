use std::fs;

use parser::class_file_reader::ClassFileReader;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use vm::class_loader::class_loader::ClassLoader;
use vm::vis;
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
        "vis" => vis(&args[2].as_str()).await,
        cmd => panic!("command {} not implemented yet.", cmd),
    }
}

async fn vis(class: &str) {
    let mut vm = VM::new(4).await;
    let main_class = add_prepare(&class, &mut vm);
    let class_name = main_class.clone();
    let _ = vm.class_loader.add_directory_entry("".to_string());
    /*
    let producer = tokio::spawn(async {
        vm.invoke_main(&class_name).await;
    });

    let consumer = tokio::spawn(async {
        vis::consumer_thread().await;
    });

    let _ = tokio::try_join!(producer, consumer);
*/
        let _ = vm.invoke_main(&class_name).await;
        vis::consumer_thread().await;
}

async fn run(class: &str) {
    //   let mut vm = VM::new();
    // TODO provide args as string array
    // let start_args = vec![Value::Int(0)];
    //  vm.start(main_class, start_args);
    //let class = class_manager.get_or_resolve_class(main_class).unwrap();
    //    println!("{class}");
    let mut vm = VM::new(4).await;
    let main_class = add_prepare(&class, &mut vm);
    let _ = vm.class_loader.add_directory_entry("".to_string());
    //let _ = vm.class_loader.add_directory_entry("../Temp/java/".to_string());
    let damn = vm.invoke_main(&main_class).await;
    //println!("{:?}",damn);

    //dbg!(class);
}

fn add_prepare(unprepared: &str, vm: &mut VM) -> String {
    let trimmed_unprepared = if unprepared.ends_with(".class") {
        &unprepared[..unprepared.len() - 6]
    } else {
        unprepared
    };
    if let Some(pos) = trimmed_unprepared.rfind("/") {
        let main_class = &trimmed_unprepared[pos + 1..];
        let entry = &trimmed_unprepared[..pos + 1];
        let _ = vm.class_loader.add_directory_entry(entry.to_string());
        main_class.to_string()
    } else {
        trimmed_unprepared.to_string()
    }
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
