// mod arranger;
mod functions;
mod syntax;

// use arranger::Arranger;
use functions::{draw, expand, pretty, quadratic::Quadratic};
use std::env;
use std::fs;
use syntax::parser::{Parser, AST};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parser = Parser::new(fs::read_to_string(args[1].clone()).unwrap());
    let ast = parser.parse();
    let mut draws: Vec<(AST, String)> = Vec::new();
    for statement in ast {
        println!("{}", pretty(*statement.clone()));
        match *statement {
            AST::Statement(a) => {
                if a.clone().command.unwrap_or(String::new()) == "draw".to_string() {
                    let expr = expand(AST::Statement(a));
                    let label = pretty(expr.clone());
                    println!("{}", label);
                    let expr = Quadratic::from(expr, "y".to_string()).solve();
                    println!("{}", pretty(expr.clone()));
                    let expr = expand(expr);
                    println!("{}", pretty(expr.clone()));
                    draws.push((expr, label));
                }
            }
            _ => {}
        }
    }
    draw(&args[2].clone(), draws, &args[3].clone()).expect("Failed to draw.");
}
