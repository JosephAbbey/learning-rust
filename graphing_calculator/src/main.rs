mod cmd;
mod functions;
mod syntax;

use cmd::run;
use functions::{draw, eval, expand, pretty, quadratic::Quadratic};
use syntax::parser::{Identity, Parser, AST};

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    fn graph(args: Vec<String>) {
        let mut parser = Parser::new(fs::read_to_string(args[0].clone()).unwrap());
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
        draw(&args[1].clone(), draws, &args[2].clone()).expect("Failed to draw.");
    }

    fn solve(args: Vec<String>) {
        let mut parser = Parser::new(args[0].clone() + ";");
        let ast = parser.parse();
        println!("original:  {}", pretty(*ast[0].clone()));
        match *ast[0].clone() {
            AST::Statement(a) => {
                let expr = expand(AST::Statement(a));
                let label = pretty(expr.clone());
                println!("expanded:  {}", label);
                let quad = Quadratic::from(expr, args[1].clone());
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
            }
            _ => {}
        }
    }

    fn simultaneous(args: Vec<String>) {
        let mut expr1: Option<AST> = None;
        let mut parser = Parser::new(args[0].clone() + ";");
        let ast = parser.parse();
        println!("original:  {}", pretty(*ast[0].clone()));
        match *ast[0].clone() {
            AST::Statement(a) => {
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
                expr1 = Some(expand(expr));
                println!("expanded:  {}", pretty(expr1.clone().unwrap()));
                println!("");
            }
            _ => {}
        }

        let mut expr2: Option<AST> = None;
        let mut parser = Parser::new(args[1].clone() + ";");
        let ast = parser.parse();
        println!("original:  {}", pretty(*ast[0].clone()));
        match *ast[0].clone() {
            AST::Statement(a) => {
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
                expr2 = Some(expand(expr));
                println!("expanded:  {}", pretty(expr2.clone().unwrap()));
                println!("");
            }
            _ => {}
        }

        let expr = AST::Identity(Identity {
            identity: vec![
                match expr1.clone().unwrap() {
                    AST::Identity(a) => a.identity[1].clone(),
                    _ => panic!("Expected identity."),
                },
                match expr2.unwrap() {
                    AST::Identity(a) => a.identity[1].clone(),
                    _ => panic!("Expected identity."),
                },
            ],
        });
        println!("original: {}", pretty(expr.clone()));
        let quad = Quadratic::from(expr, "x".to_string());
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
        let xs = eval(expr, 0f64);
        println!("evaluated: x = {:?}", xs.clone());
        let mut ys: Vec<f64> = Vec::new();
        for x in xs.clone() {
            ys.extend(eval(expr1.clone().unwrap(), x));
        }
        println!("evaluated: y = {:?}", ys);
        let mut i = 0;
        println!(
            "points: {}",
            xs.iter()
                .map(|x| {
                    i += 1;
                    format!("({:.3}, {:.3})", x, ys[i - 1])
                })
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!("");
    }

    run(
        args,
        vec![
            "draw".to_string(),
            "solve".to_string(),
            "simultaneous".to_string(),
        ],
        vec![&graph, &solve, &simultaneous],
    );
}
