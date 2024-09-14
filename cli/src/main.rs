use std::fs;

use parser::class_file_reader::ClassFileReader;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        panic!("Invalid args\n usage: <exe> <class_file>");
    }

    println!("parsing class file {}", &args[1]);

    //TODO: add size bound
    //read whole file to memory as Vec<u8>
    let content = fs::read(&args[1]).unwrap();

    let reader = ClassFileReader::new(content);
    let class_file = reader.parse();
    dbg!(class_file.unwrap());

    println!("Parsing completed");
}
