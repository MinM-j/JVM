use std::fs::File;

use vm::class_file::ClassFile;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        panic!("Invalid args\n usage: <exe> <class_file>");
    }

    println!("parsing class file {}", &args[1]);

    let file = File::open(&args[1]).unwrap();

    ClassFile::parse(file);

    println!("Parsing completed");
}
