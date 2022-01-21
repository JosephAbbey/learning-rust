mod functions;
mod syntax;

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
        println!("original:  {}", pretty(*statement.clone()));
        match *statement {
            AST::Statement(a) => {
                if a.clone().command.unwrap_or(String::new()) == "draw".to_string() {
                    let expr = expand(AST::Statement(a));
                    let label = pretty(expr.clone());
                    println!("expanded:  {}", label);
                    let quad = Quadratic::from(expr, "y".to_string());
                    println!(
                        "quadratic: \n  a: {}\n  b: {}\n  c: {}",
                        pretty(quad.a.clone()),
                        pretty(quad.b.clone()),
                        pretty(quad.c.clone())
                    );
                    let expr = quad.solve();
                    println!("solved:    {}", pretty(expr.clone()));
                    let expr = expand(expr);
                    println!("expanded:  {}", pretty(expr.clone()));
                    draws.push((expr, label));
                    println!("");
                }
            }
            _ => {}
        }
    }
    draw(&args[2].clone(), draws, &args[3].clone()).expect("Failed to draw.");
}
