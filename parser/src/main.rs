use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let source = fs::read_to_string(file_path)
        .expect("Source file unreadable");


    println!(
        "I'm using the library: {:?}",
        emerald_lang_parser::parser::parse_program(&source, &file_path)
    );
}

