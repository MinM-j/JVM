use std::fs;

use parser::class_file_reader::ClassFileReader;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use vm::class_loader::class_loader::ClassLoader;
use vm::state::{FILE_NAME, MEMORY_SIZE, MEMORY_SNAP, VIS_BOOL};
use vm::vis;
use vm::vm::VM;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        panic!("Invalid args\n usage: <exe> [ parse | run | vis] <class_file>");
    }

    if let Some(mem_pos) = args.iter().position(|arg| arg == "--mem") {
        if mem_pos + 1 < args.len() {
            if let Ok(mem_value) = args[mem_pos + 1].parse::<usize>() {
                let mut mem_size = MEMORY_SIZE.lock().unwrap();
                *mem_size = mem_value;
            } else {
                panic!("Invalid value for mem. Please provide an integer.");
            }
        } else {
            panic!("--mem flag requires a value.");
        }
    }

    if let Some(file_pos) = args.iter().position(|arg| arg == "--file") {
        if file_pos + 1 < args.len() {
            if args.contains(&"--vis".to_string()) {
                let file_value = &args[file_pos + 1];
                if file_value.ends_with(".json") {
                    let mut file_name = FILE_NAME.lock().unwrap();
                    *file_name = file_value.clone();
                } else {
                    panic!("--file value must end with .json.");
                }
            } else {
                panic!("--file flag requires --vis to be present.");
            }
        } else {
            panic!("--file flag requires a value.");
        }
    }

    if args.iter().any(|arg| arg == "--snap") {
        if args.contains(&"--vis".to_string()) {
            let mut snap = MEMORY_SNAP.lock().unwrap();
            *snap = true;
        } else {
            panic!("--file flag requires --vis to be present.");
        }
    }

    match args.get(1).unwrap().as_str() {
        "--parse" => parse(&args[2].as_str()),
        "--run" => run(&args[2].as_str()).await,
        "--vis" => {
            {
                let mut vis_flag = VIS_BOOL.lock().unwrap();
                *vis_flag = true;
            }
            vis(&args[2].as_str()).await
        }
        cmd => panic!("command {} not implemented yet.", cmd),
    }
}

async fn vis(class: &str) {
    let mem_size = MEMORY_SIZE.lock().unwrap();
    let mut vm = VM::new(*mem_size).await;
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
    //vis::consumer_thread().await;
    vis::file_writer().await;
}

async fn run(class: &str) {
    //   let mut vm = VM::new();
    // TODO provide args as string array
    // let start_args = vec![Value::Int(0)];
    //  vm.start(main_class, start_args);
    //let class = class_manager.get_or_resolve_class(main_class).unwrap();
    //    println!("{class}");
    let mem_size = MEMORY_SIZE.lock().unwrap();
    let mut vm = VM::new(*mem_size).await;
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
