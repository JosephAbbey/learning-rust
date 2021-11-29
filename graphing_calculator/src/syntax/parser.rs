use super::lexer::{Lexer, Token, TokenKind};

pub trait AST {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

impl std::fmt::Debug for dyn AST {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.fmt(f)
  }
}

struct BinOp {
  token: Token,
  left: Option<Box<dyn AST>>,
  right: Option<Box<dyn AST>>,
}

impl BinOp {
  pub fn new(left: Option<Box<dyn AST>>, token: Token, right: Option<Box<dyn AST>>) -> BinOp {
    BinOp { left, token, right }
  }
}

impl AST for BinOp {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "({:#?} {} {:#?})",
      self.left, self.token.value, self.right
    )
  }
}

struct UnaryOp {
  token: Token,
  expr: Option<Box<dyn AST>>,
}

impl UnaryOp {
  pub fn new(token: Token, expr: Option<Box<dyn AST>>) -> UnaryOp {
    UnaryOp { token, expr }
  }
}

impl AST for UnaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({} {:#?})", self.token.value, self.expr)
  }
}

struct Number {
  token: Token,
  value: f32,
}

impl Number {
  pub fn new(token: Token) -> Number {
    Number {
      token: token.clone(),
      value: token.value.parse::<f32>().unwrap(),
    }
  }
}

impl AST for Number {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({})", self.value)
  }
}

struct Variable {
  token: Token,
  name: char,
}

impl Variable {
  pub fn new(token: Token) -> Variable {
    Variable {
      token: token.clone(),
      name: token.value.chars().next().unwrap(),
    }
  }
}

impl AST for Variable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

struct Command {
  token: Token,
  name: String,
}

impl Command {
  pub fn new(token: Token, name: String) -> Command {
    Command { token, name }
  }
}

impl AST for Command {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "")
  }
}

struct Assign {
  token: Token,
  left: Option<Box<dyn AST>>,
  right: Option<Box<dyn AST>>,
}

impl Assign {
  pub fn new(left: Option<Box<dyn AST>>, token: Token, right: Option<Box<dyn AST>>) -> Assign {
    Assign { left, token, right }
  }
}

impl AST for Assign {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "({:#?} {} {:#?})",
      self.left, self.token.value, self.right
    )
  }
}

struct Statement {
  equation: Option<Box<dyn AST>>,
  command: Option<Box<dyn AST>>,
}

impl Statement {
  pub fn new<'a>(equation: Option<Box<dyn AST>>, command: Option<Box<dyn AST>>) -> Statement {
    Statement { equation, command }
  }
}

impl AST for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:#?}", self.equation)
  }
}

pub struct Parser {
  lexer: Lexer,
  current_token: Token,
}

// for every AST use 'a to stop lifetime mismatch
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

  fn factor(&mut self) -> Option<Box<dyn AST>> {
    let token = self.current_token.clone();
    if token.kind == TokenKind::Add {
      self.eat(TokenKind::Add);
      return Some(Box::new(UnaryOp::new(token, self.factor())));
    } else if token.kind == TokenKind::Subtract {
      self.eat(TokenKind::Subtract);
      return Some(Box::new(UnaryOp::new(token, self.factor())));
    } else if token.kind == TokenKind::Number {
      self.eat(TokenKind::Number);
      return Some(Box::new(Number::new(token)));
    } else if token.kind == TokenKind::Variable {
      self.eat(TokenKind::Variable);
      return Some(Box::new(Variable::new(token)));
    } else if token.kind == TokenKind::LeftParen {
      self.eat(TokenKind::LeftParen);
      let node = self.expr();
      self.eat(TokenKind::RightParen);
      return node;
    }
    None
  }

  fn term(&mut self) -> Option<Box<dyn AST>> {
    let mut node = self.factor();

    while [TokenKind::Multiply, TokenKind::Divide, TokenKind::Variable]
      .contains(&self.current_token.kind)
    {
      let mut token = self.current_token.clone();
      if token.kind == TokenKind::Multiply {
        self.eat(TokenKind::Multiply);
      } else if token.kind == TokenKind::Divide {
        self.eat(TokenKind::Divide);
      } else if token.kind == TokenKind::Variable {
        token = Token::new(TokenKind::Multiply, &"", token.position.clone());
      }
      node = Some(Box::new(BinOp::new(node, token, self.factor())));
    }

    node
  }

  fn expr(&mut self) -> Option<Box<dyn AST>> {
    if self.current_token.kind == TokenKind::Semicolon {
      return None;
    }
    let mut node = self.term();
    while [TokenKind::Add, TokenKind::Subtract].contains(&self.current_token.kind) {
      let token = self.current_token.clone();
      if token.kind == TokenKind::Add {
        self.eat(TokenKind::Add);
      } else if token.kind == TokenKind::Subtract {
        self.eat(TokenKind::Subtract);
      }
      node = Some(Box::new(BinOp::new(node, token, self.term())));
    }

    node
  }

  fn command(&mut self) -> Option<Box<dyn AST>> {
    let token = self.current_token.clone();
    if token.kind == TokenKind::Command {
      self.eat(TokenKind::Command);
      let token_id = self.current_token.clone();
      self.eat(TokenKind::Identifier);
      return Some(Box::new(Command::new(
        token.clone(),
        token_id.value.to_string(),
      )));
    }
    None
  }

  fn assign(&mut self) -> Option<Box<dyn AST>> {
    let left = self.expr();
    let token = self.current_token.clone();
    self.eat(TokenKind::Assign);
    let right = self.expr();
    Some(Box::new(Assign::new(left, token, right)))
  }

  fn statement(&mut self) -> Option<Box<dyn AST>> {
    let node = self.assign();
    self.eat(TokenKind::Semicolon);
    Some(Box::new(Statement::new(node, self.command())))
  }

  fn statements(&mut self) -> Vec<Option<Box<dyn AST>>> {
    let mut statements = Vec::new();
    while self.current_token.kind != TokenKind::EOF {
      statements.push(self.statement());
    }
    statements
  }

  pub fn parse(&mut self) -> Vec<Option<Box<dyn AST>>> {
    self.statements()
  }
}
