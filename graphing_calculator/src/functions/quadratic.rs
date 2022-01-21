use super::{eval, expand};
use crate::syntax::parser::{Call, Expr, Identity, Index, Sign, Term, Unary, AST};

#[derive(Debug, Clone)]
pub struct Quadratic {
  pub a: AST,
  pub b: AST,
  pub c: AST,
  var: String,
}

impl Quadratic {
  pub fn new(a: AST, b: AST, c: AST, var: String) -> Quadratic {
    Quadratic { a, b, c, var }
  }

  pub fn from(ast: AST, var: String) -> Quadratic {
    match ast {
      AST::Expr(expr) => {
        let mut quadratic = Quadratic::new(
          AST::Number(0f64),
          AST::Number(0f64),
          AST::Number(0f64),
          var.clone(),
        );
        for child in expr.expr {
          match *child {
            AST::Term(term) => {
              if term.term.contains(&Box::new(AST::Index(Index {
                sign: Sign::Pow,
                index: (
                  Box::new(AST::Variable(var.clone())),
                  Box::new(AST::Number(2f64)),
                ),
              }))) {
                let mut t = term.clone();
                t.term.remove(
                  t.term
                    .iter()
                    .position(|x| {
                      *x == Box::new(AST::Index(Index {
                        sign: Sign::Pow,
                        index: (
                          Box::new(AST::Variable(var.clone())),
                          Box::new(AST::Number(2f64)),
                        ),
                      }))
                    })
                    .unwrap(),
                );
                if quadratic.a == AST::Number(0f64) {
                  quadratic.a = AST::Term(t);
                } else {
                  quadratic.a = AST::Expr(Expr {
                    sign: Sign::Add,
                    expr: vec![Box::new(quadratic.a), Box::new(AST::Term(t))],
                  });
                }
              } else if term.term.contains(&Box::new(AST::Variable(var.clone()))) {
                let mut t = term.clone();
                t.term.remove(
                  t.term
                    .iter()
                    .position(|x| *x == Box::new(AST::Variable(var.clone())))
                    .unwrap(),
                );
                if quadratic.b == AST::Number(0f64) {
                  quadratic.b = AST::Term(t);
                } else {
                  quadratic.b = AST::Expr(Expr {
                    sign: Sign::Add,
                    expr: vec![Box::new(quadratic.b), Box::new(AST::Term(t))],
                  });
                }
              } else {
                if quadratic.c == AST::Number(0f64) {
                  quadratic.c = AST::Term(term);
                } else {
                  quadratic.c = AST::Expr(Expr {
                    sign: Sign::Add,
                    expr: vec![Box::new(quadratic.c), Box::new(AST::Term(term))],
                  });
                }
              }
            }
            AST::Variable(v) => {
              if v != var.clone() {
                if quadratic.c == AST::Number(0f64) {
                  quadratic.c = AST::Variable(v).clone();
                } else {
                  quadratic.c = AST::Expr(Expr {
                    sign: Sign::Add,
                    expr: vec![Box::new(quadratic.c), Box::new(AST::Variable(v))],
                  });
                }
              }
            }
            AST::Number(n) => {
              if quadratic.c == AST::Number(0f64) {
                quadratic.c = AST::Number(n).clone();
              } else {
                quadratic.c = AST::Expr(Expr {
                  sign: Sign::Add,
                  expr: vec![Box::new(quadratic.c), Box::new(AST::Number(n))],
                });
              }
            }
            other => {
              if quadratic.c == AST::Number(0f64) {
                quadratic.c = other.clone();
              } else {
                quadratic.c = AST::Expr(Expr {
                  sign: Sign::Add,
                  expr: vec![Box::new(quadratic.c), Box::new(other)],
                });
              }
            }
          }
        }
        quadratic
      }
      AST::Statement(statement) => Quadratic::from(*statement.statement, var),
      AST::Identity(identity) => Quadratic::one_side(
        Quadratic::from(
          match *identity.identity[0].clone() {
            AST::Expr(expr) => AST::Expr(expr),
            _ => AST::Expr(Expr {
              sign: Sign::Add,
              expr: vec![identity.identity[0].clone()],
            }),
          },
          var.clone(),
        ),
        Quadratic::from(
          match *identity.identity[1].clone() {
            AST::Expr(expr) => AST::Expr(expr),
            _ => AST::Expr(Expr {
              sign: Sign::Add,
              expr: vec![identity.identity[1].clone()],
            }),
          },
          var.clone(),
        ),
        var.clone(),
      ),
      _ => panic!("Unsupported type, got: {:?}", ast),
    }
  }

  pub fn one_side(q1: Quadratic, q2: Quadratic, var: String) -> Quadratic {
    return Quadratic::new(
      AST::Expr(Expr {
        sign: Sign::Sub,
        expr: vec![Box::new(q1.a), Box::new(q2.a)],
      }),
      AST::Expr(Expr {
        sign: Sign::Sub,
        expr: vec![Box::new(q1.b), Box::new(q2.b)],
      }),
      AST::Expr(Expr {
        sign: Sign::Sub,
        expr: vec![Box::new(q1.c), Box::new(q2.c)],
      }),
      var,
    );
  }

  pub fn solve(&self) -> AST {
    if eval(self.a.clone(), 0f64)[0] != 0f64 {
      AST::Identity(Identity {
        identity: vec![
          Box::new(AST::Variable(self.var.clone())),
          Box::new(AST::Term(Term {
            sign: Sign::Div,
            term: vec![
              Box::new(AST::Expr(Expr {
                sign: Sign::AddSub,
                expr: vec![
                  Box::new(AST::Unary(Unary {
                    sign: Sign::Sub,
                    unary: Box::new(self.b.clone()),
                  })),
                  Box::new(AST::Call(Call {
                    name: "sqrt".to_string(),
                    call: vec![Box::new(AST::Expr(Expr {
                      sign: Sign::Sub,
                      expr: vec![
                        Box::new(AST::Index(Index {
                          sign: Sign::Pow,
                          index: (Box::new(self.b.clone()), Box::new(AST::Number(2f64))),
                        })),
                        Box::new(AST::Term(Term {
                          sign: Sign::Mul,
                          term: vec![
                            Box::new(AST::Number(4f64)),
                            Box::new(self.a.clone()),
                            Box::new(self.c.clone()),
                          ],
                        })),
                      ],
                    }))],
                  })),
                ],
              })),
              Box::new(AST::Term(Term {
                sign: Sign::Mul,
                term: vec![Box::new(AST::Number(2f64)), Box::new(self.a.clone())],
              })),
            ],
          })),
        ],
      })
    } else if eval(self.b.clone(), 0f64)[0] != 0f64 {
      AST::Identity(Identity {
        identity: vec![
          Box::new(AST::Variable(self.var.clone())),
          Box::new(AST::Term(Term {
            sign: Sign::Div,
            term: vec![
              Box::new(AST::Unary(Unary {
                sign: Sign::Sub,
                unary: Box::new(self.c.clone()),
              })),
              Box::new(self.b.clone()),
            ],
          })),
        ],
      })
    } else if let AST::Number(_) = expand(self.c.clone()) {
      AST::Identity(Identity {
        identity: vec![
          Box::new(AST::Variable(self.var.clone())),
          Box::new(self.c.clone()),
        ],
      })
    } else if self.var == "y".to_string() {
      AST::Identity(Identity {
        identity: vec![
          Box::new(AST::Variable(self.var.clone())),
          Box::new(Quadratic::from(self.c.clone(), "x".to_string()).solve()),
        ],
      })
    } else {
      AST::Identity(Identity {
        identity: vec![
          Box::new(AST::Variable(self.var.clone())),
          Box::new(Quadratic::from(self.c.clone(), "y".to_string()).solve()),
        ],
      })
    }
  }
}
