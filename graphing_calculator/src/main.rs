// mod arranger;
mod functions;
mod syntax;

// use arranger::Arranger;
use functions::{expand, pretty, quadratic::Quadratic};
use std::env;
use std::fs;
use syntax::parser::{Parser, AST};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parser = Parser::new(fs::read_to_string(args[1].clone()).unwrap());
    let ast = parser.parse();
    for statement in ast {
        println!("{}", pretty(*statement.clone()));
        match *statement {
            AST::Statement(a) => {
                if a.clone().command.unwrap_or(String::new()) == "draw".to_string() {
                    println!(
                        "{}",
                        pretty(Quadratic::from(expand(AST::Statement(a))).solve())
                    );
                }
            }
            _ => {}
        }
    }
}
