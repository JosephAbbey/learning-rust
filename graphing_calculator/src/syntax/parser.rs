use super::lexer::{Lexer, Token, TokenKind};

#[derive(Debug, Clone)]
pub enum AST {
  Expr(Expr),
  Term(Term),
  Index(Index),
  Unary(Unary),
  Variable(String),
  Number(f64),
  Call(Call),
  // temp
  None,
}

#[derive(Debug, Clone)]
enum Sign {
  Add,
  Sub,
  AddSub,
  Mul,
  Div,
  Pow,
}

#[derive(Debug, Clone)]
struct Expr {
  sign: Sign,
  expr: Vec<Box<AST>>,
}

#[derive(Debug, Clone)]
struct Term {
  sign: Sign,
  term: Vec<Box<AST>>,
}

#[derive(Debug, Clone)]
struct Index {
  sign: Sign,
  index: (Box<AST>, Box<AST>),
}

#[derive(Debug, Clone)]
struct Unary {
  sign: Sign,
  unary: Box<AST>,
}

#[derive(Debug, Clone)]
struct Call {
  name: String,
  call: Vec<Box<AST>>,
}

#[derive(Debug, Clone)]
struct Identity {
  identity: Vec<Box<AST>>,
}

#[derive(Debug, Clone)]
struct Statement {
  identity: Box<AST>,
  command: String,
}

pub struct Parser {
  lexer: Lexer,
  current_token: Token,
}

impl Parser {
  pub fn new(input: String) -> Parser {
    let mut lexer = Lexer::new(input);
    let current_token = lexer.get_next_token();
    Parser {
      lexer,
      current_token,
    }
  }

  // method to call for lexing errors
  fn error(&self, msg: String) -> ! {
    panic!("{}", msg)
  }

  fn eat(&mut self, token_type: TokenKind) {
    if self.current_token.kind == token_type {
      self.current_token = self.lexer.get_next_token();
    } else {
      self.error(format!(
        "SyntaxError: Unexpected {:?}: '{}' at position {}:{} expecting {:?}",
        self.current_token.kind.clone(),
        self.current_token.value.clone(),
        self.current_token.position.human.line.clone(),
        self.current_token.position.human.column.clone(),
        token_type,
      ));
    }
  }

  fn factor(&mut self) -> Box<AST> {
    let token = self.current_token.clone();
    if token.kind == TokenKind::Add {
      self.eat(TokenKind::Add);
      return Box::new(AST::Unary(Unary {
        sign: Sign::Add,
        unary: self.factor(),
      }));
    } else if token.kind == TokenKind::Subtract {
      self.eat(TokenKind::Subtract);
      return Box::new(AST::Unary(Unary {
        sign: Sign::Sub,
        unary: self.factor(),
      }));
    } else if token.kind == TokenKind::Number {
      self.eat(TokenKind::Number);
      return Box::new(AST::Number(token.value.parse().unwrap()));
    } else if token.kind == TokenKind::Identifier {
      self.eat(TokenKind::Identifier);
      if self.current_token.kind == TokenKind::LeftParen {
        self.eat(TokenKind::LeftParen);
        let mut call = Vec::new();
        while self.current_token.kind != TokenKind::RightParen {
          call.push(self.expr());
          if self.current_token.kind == TokenKind::Comma {
            self.eat(TokenKind::Comma);
          }
        }
        self.eat(TokenKind::RightParen);
        return Box::new(AST::Call(Call {
          name: token.value.clone(),
          call,
        }));
      } else {
        return Box::new(AST::Variable(token.value));
      }
    } else if token.kind == TokenKind::LeftParen {
      self.eat(TokenKind::LeftParen);
      let node = self.expr();
      self.eat(TokenKind::RightParen);
      return node;
    }
    println!("{:?}", token);
    self.error(format!(
      "SyntaxError: Unexpected {:?}: '{}' at position {}:{}",
      self.current_token.kind.clone(),
      self.current_token.value.clone(),
      self.current_token.position.human.line.clone(),
      self.current_token.position.human.column.clone(),
    ));
  }

  fn term(&mut self) -> Box<AST> {
    let mut node = self.factor();

    while [
      TokenKind::Multiply,
      TokenKind::Divide,
      TokenKind::LeftParen,
      TokenKind::Identifier,
      TokenKind::Number,
    ]
    .contains(&self.current_token.kind)
    {
      let mut token = self.current_token.clone();
      if token.kind == TokenKind::Multiply {
        self.eat(TokenKind::Multiply);
      } else if token.kind == TokenKind::Divide {
        self.eat(TokenKind::Divide);
      } else if [
        TokenKind::LeftParen,
        TokenKind::Identifier,
        TokenKind::Number,
      ]
      .contains(&self.current_token.kind)
      {
        token = Token::new(TokenKind::Multiply, &"", token.position.clone());
      }
      node = match *node {
        AST::Term(n) => {
          let mut n = n.clone();
          n.term.push(self.factor());
          Box::new(AST::Term(n))
        }
        _ => Box::new(AST::Term(Term {
          sign: match token.kind {
            TokenKind::Multiply => Sign::Mul,
            TokenKind::Divide => Sign::Div,
            _ => {
              self.error(format!("SyntaxError: Unexpected {:?}", token.kind));
            }
          },
          term: vec![node, self.factor()],
        })),
      }
    }

    node
  }

  fn expr(&mut self) -> Box<AST> {
    Box::new(AST::None)
  }

  fn identity(&mut self) -> Box<AST> {
    Box::new(AST::None)
  }

  fn statement(&mut self) -> Box<AST> {
    Box::new(AST::None)
  }

  fn statements(&mut self) -> Vec<Box<AST>> {
    vec![]
  }

  pub fn parse(&mut self) -> Vec<Box<AST>> {
    self.statements()
  }
}
