use crate::syntax::parser::{Call, Expr, Identity, Sign, Term, AST};

pub fn expand(ast: AST) -> AST {
  match ast.clone() {
    AST::Unary(u) => {
      let n = expand(*u.unary.clone());
      match u.sign {
        Sign::Add => n,
        Sign::Sub => match n {
          AST::Number(n) => AST::Number(-n),
          _ => AST::Unary(u),
        },
        _ => AST::Unary(u),
      }
    }
    AST::Expr(e) => {
      let mut o: Vec<Box<AST>> = Vec::new();
      let mut n = 0f64;
      fn expand_expr(e: Expr, mut o: &mut Vec<Box<AST>>, mut n: &mut f64) {
        for i in e.expr {
          let j = expand(*i);
          match j {
            AST::Number(x) => match e.sign {
              Sign::Add => *n += x,
              Sign::Sub => *n += x,
              Sign::AddSub => o.push(Box::new(j.clone())),
              _ => {}
            },
            AST::Expr(x) => {
              if e.sign == Sign::Add {
                expand_expr(x, &mut o, &mut n)
              } else {
                o.push(Box::new(expand(AST::Expr(x))))
              }
            }
            _ => o.push(Box::new(j.clone())),
          };
        }
      }
      expand_expr(e.clone(), &mut o, &mut n);
      if n != 0f64 {
        o.splice(0..0, [Box::new(AST::Number(n))]);
      }
      if o.len() == 0 {
        AST::Number(0f64)
      } else if o.len() == 1 {
        *o.get(0).unwrap().clone()
      } else {
        AST::Expr(Expr {
          expr: o,
          sign: e.sign,
        })
      }
    }
    AST::Term(t) => {
      let mut o: Vec<Box<AST>> = Vec::new();
      let mut n = 1f64;
      fn expand_term(t: Term, mut o: &mut Vec<Box<AST>>, mut n: &mut f64) {
        for i in t.term {
          let j = expand(*i);
          match j {
            AST::Number(x) => match t.sign {
              Sign::Mul => *n *= x,
              _ => o.push(Box::new(j)),
            },
            AST::Term(x) => {
              if t.sign == Sign::Mul {
                expand_term(x, &mut o, &mut n)
              } else {
                o.push(Box::new(expand(AST::Term(x))))
              }
            }
            _ => o.push(Box::new(j)),
          };
        }
      }
      expand_term(t.clone(), &mut o, &mut n);
      if n != 1f64 {
        o.push(Box::new(AST::Number(n)));
      }
      if o.len() == 1 {
        *o.get(0).unwrap().clone()
      } else {
        AST::Term(Term {
          term: o,
          sign: t.sign,
        })
      }
    }
    AST::Index(e) => {
      let i = expand(*e.index.0.clone());
      let j = expand(*e.index.1.clone());
      match i {
        AST::Number(x) => match j {
          AST::Number(y) => AST::Number(f64::powf(x, y)),
          _ => AST::Index(e),
        },
        _ => AST::Index(e),
      }
    }
    AST::Identity(s) => AST::Identity(Identity {
      identity: s
        .identity
        .iter()
        .map(|i| Box::new(expand(*i.clone())))
        .collect::<Vec<_>>(),
    }),
    AST::Statement(s) => expand(*s.statement),
    AST::Call(c) => {
      let a = expand(*c.call[0].clone());
      match a {
        AST::Number(x) => match c.name.as_str() {
          "sqrt" => AST::Number(f64::sqrt(x)),
          "sin" => AST::Number(f64::sin(x)),
          "cos" => AST::Number(f64::cos(x)),
          "tan" => AST::Number(f64::tan(x)),
          "asin" => AST::Number(f64::asin(x)),
          "acos" => AST::Number(f64::acos(x)),
          "atan" => AST::Number(f64::atan(x)),
          "ln" => AST::Number(f64::ln(x)),
          _ => AST::Call(Call {
            call: vec![Box::new(a)],
            name: c.name,
          }),
        },
        _ => AST::Call(Call {
          call: vec![Box::new(a)],
          name: c.name,
        }),
      }
    }
    _ => ast,
  }
}
