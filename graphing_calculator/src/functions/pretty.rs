use crate::syntax::parser::{Sign, AST};

pub fn pretty(ast: AST) -> String {
  let mut out = String::new();
  match ast {
    AST::Number(n) => out.push_str(&n.to_string()),
    AST::Variable(v) => out.push_str(&v.to_string()),
    AST::Expr(e) => {
      if e.expr.len() != 0 {
        out.push_str("(");
        let f = &pretty(*e.expr[0].clone());
        if f.len() == 0 {
          out.push_str("0");
        } else {
          out.push_str(f);
        }
        for i in 1..e.expr.len() {
          out.push_str(" ");
          out.push_str(match e.sign {
            Sign::Add => "+",
            Sign::Sub => "-",
            Sign::AddSub => "±",
            Sign::Mul => "*",
            Sign::Div => "/",
            Sign::Pow => "^",
          });
          out.push_str(" ");
          out.push_str(&pretty(*e.expr[i].clone()));
        }
        out.push_str(")");
      }
    }
    AST::Term(t) => {
      out.push_str("(");
      let f = &pretty(*t.term[0].clone());
      if f.len() == 0 {
        out.push_str("0");
      } else {
        out.push_str(f);
      }
      for i in 1..t.term.len() {
        out.push_str(" ");
        out.push_str(match t.sign {
          Sign::Add => "+",
          Sign::Sub => "-",
          Sign::AddSub => "±",
          Sign::Mul => "*",
          Sign::Div => "/",
          Sign::Pow => "^",
        });
        out.push_str(" ");
        out.push_str(&pretty(*t.term[i].clone()));
      }
      out.push_str(")");
    }
    AST::Index(i) => {
      out.push_str("(");
      out.push_str(&pretty(*i.index.0.clone()));
      out.push_str(" ");
      out.push_str(match i.sign {
        Sign::Add => "+",
        Sign::Sub => "-",
        Sign::AddSub => "±",
        Sign::Mul => "*",
        Sign::Div => "/",
        Sign::Pow => "^",
      });
      out.push_str(" ");
      out.push_str(&pretty(*i.index.1.clone()));
      out.push_str(")");
    }
    AST::Unary(u) => {
      out.push_str("(");
      out.push_str(match u.sign {
        Sign::Add => "+",
        Sign::Sub => "-",
        Sign::AddSub => "±",
        Sign::Mul => "*",
        Sign::Div => "/",
        Sign::Pow => "^",
      });
      out.push_str(&pretty(*u.unary.clone()));
      out.push_str(")");
    }
    AST::Identity(i) => {
      out.push_str(&pretty(*i.identity[0].clone()));
      for j in 1..i.identity.len() {
        out.push_str(" = ");
        out.push_str(&pretty(*i.identity[j].clone()));
      }
    }
    AST::Call(c) => {
      out.push_str(&c.name);
      out.push_str("(");
      out.push_str(&pretty(*c.call[0].clone()));
      for j in 1..c.call.len() {
        out.push_str(", ");
        out.push_str(&pretty(*c.call[j].clone()));
      }
      out.push_str(")");
    }
    AST::Statement(s) => {
      out.push_str(&pretty(*s.statement.clone()));
    }
  }
  out
}
