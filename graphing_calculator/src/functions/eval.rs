use crate::syntax::parser::{Sign, AST};

pub fn eval(expr: AST, x: f64) -> Vec<f64> {
  match expr.clone() {
    AST::Number(n) => vec![n],
    AST::Variable(v) => match v.as_str() {
      "x" => vec![x],
      _ => panic!("Unknown variable {}", v),
    },
    AST::Unary(u) => {
      let mut o = Vec::<f64>::new();
      for n in eval(*u.unary.clone(), x) {
        o.extend(match u.sign {
          Sign::Add => vec![n],
          Sign::Sub => vec![-n],
          Sign::AddSub => vec![n, -n],
          _ => vec![0f64],
        });
      }
      o
    }
    AST::Expr(e) => {
      let mut o = eval(*e.expr[0].clone(), x);
      for i in 1..e.expr.len() {
        let oc = o.clone();
        o.clear();
        for j in oc {
          for n in eval(*e.expr[i].clone(), x) {
            o.extend(match e.sign {
              Sign::Add => vec![j + n],
              Sign::Sub => vec![j - n],
              Sign::AddSub => vec![j + n, j - n],
              Sign::Mul => vec![j * n],
              Sign::Div => vec![j / n],
              _ => vec![0f64],
            });
          }
        }
      }
      o
    }
    AST::Term(t) => {
      let mut o = eval(*t.term[0].clone(), x);
      for i in 1..t.term.len() {
        let oc = o.clone();
        o.clear();
        for j in oc {
          for n in eval(*t.term[i].clone(), x) {
            o.extend(match t.sign {
              Sign::Add => vec![j + n],
              Sign::Sub => vec![j - n],
              Sign::AddSub => vec![j + n, j - n],
              Sign::Mul => vec![j * n],
              Sign::Div => vec![j / n],
              _ => vec![0f64],
            });
          }
        }
      }
      o
    }
    AST::Index(e) => match e.sign {
      Sign::Pow => {
        let mut o = Vec::<f64>::new();
        for i in eval(*e.index.0.clone(), x) {
          for j in eval(*e.index.1.clone(), x) {
            o.push(f64::powf(i, j));
          }
        }
        o
      }
      _ => vec![0f64],
    },
    AST::Identity(s) => eval(*s.identity[1].clone(), x),
    AST::Statement(s) => eval(*s.statement, x),
    AST::Call(c) => match c.name.as_str() {
      "sqrt" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::sqrt(n));
        }
        o
      }
      "sin" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::sin(n));
        }
        o
      }
      "cos" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::cos(n));
        }
        o
      }
      "tan" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::tan(n));
        }
        o
      }
      "asin" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::asin(n));
        }
        o
      }
      "acos" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::acos(n));
        }
        o
      }
      "atan" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::atan(n));
        }
        o
      }
      "ln" => {
        let mut o = Vec::<f64>::new();
        for n in eval(*c.call[0].clone(), x) {
          o.push(f64::ln(n));
        }
        o
      }
      _ => vec![0f64],
    },
  }
}
