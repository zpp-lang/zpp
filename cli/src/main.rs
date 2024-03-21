use std::fs::File;
use clap::Arg;
use std::io::Read;
use std::path::Path;
use compiler::backend::lexer::Lexer;
use compiler::backend::parser::Parser;

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file.");

    contents
}

fn main() {
    let cmd = clap::Command::new("zxx")
        .bin_name("zxx")
        .arg(Arg::new("file")
            .index(1)
            .required(true))
            .arg_required_else_help(true)
        .get_matches();

    let name = cmd.get_one::<String>("file")
        .expect("No file argument provided");

    let contents = read_file(name);
    let lexer = Lexer::new(name, &contents);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();
}