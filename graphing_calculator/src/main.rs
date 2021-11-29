mod syntax;

use std::env;
use std::fs;
use syntax::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parser = Parser::new(fs::read_to_string(args[1].clone()).unwrap());
    println!("{:#?}", parser.parse());
}
