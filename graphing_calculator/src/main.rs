// mod arranger;
mod syntax;

// use arranger::Arranger;
use std::env;
use std::fs;
use syntax::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parser = Parser::new(fs::read_to_string(args[1].clone()).unwrap());
    let ast = parser.parse();
    println!("{:#?}", ast);
    // let arranger = Arranger::new(ast);
    // println!("{:#?}", arranger.arrange());
}
