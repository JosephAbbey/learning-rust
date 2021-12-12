use super::lexer::{Lexer, Token, TokenKind};

pub trait AST {
  fn token(&self) -> Option<Token>;
  fn left(&self) -> Option<Box<dyn AST>>;
  fn right(&self) -> Option<Box<dyn AST>>;
  fn kind(&self) -> String;
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
  fn _clone(&self) -> Box<dyn AST>;
}

impl std::clone::Clone for Box<dyn AST> {
  fn clone(&self) -> Box<dyn AST> {
    self._clone()
  }
}

impl std::fmt::Debug for dyn AST {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.fmt(f)
  }
}

pub struct BinOp {
  token: Token,
  left: Box<dyn AST>,
  right: Box<dyn AST>,
}

impl BinOp {
  pub fn new(left: Box<dyn AST>, token: Token, right: Box<dyn AST>) -> BinOp {
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

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    Some(self.left.clone())
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    Some(self.right.clone())
  }

  fn kind(&self) -> String {
    "BinOp".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(BinOp::new(
      self.left.clone(),
      self.token.clone(),
      self.right.clone(),
    ))
  }
}

struct UnaryOp {
  token: Token,
  expr: Box<dyn AST>,
}

impl UnaryOp {
  pub fn new(token: Token, expr: Box<dyn AST>) -> UnaryOp {
    UnaryOp { token, expr }
  }
}

impl AST for UnaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({} {:#?})", self.token.value, self.expr)
  }

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn kind(&self) -> String {
    "UnaryOp".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(UnaryOp::new(self.token.clone(), self.expr.clone()))
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

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn kind(&self) -> String {
    "Number".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(Number::new(self.token.clone()))
  }
}

pub struct Variable {
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

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn kind(&self) -> String {
    "Variable".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(Variable::new(self.token.clone()))
  }
}

struct Call {
  token: Token,
  name: String,
  args: Vec<Box<dyn AST>>,
}

impl Call {
  pub fn new(token: Token, args: Vec<Box<dyn AST>>) -> Call {
    Call {
      token: token.clone(),
      name: token.value,
      args: args,
    }
  }
}

impl AST for Call {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}({:#?})", self.name, self.args)
  }

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn kind(&self) -> String {
    "Call".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(Call::new(self.token.clone(), self.args.clone()))
  }
}

#[derive(Clone)]
pub struct Command {
  token: Token,
  pub name: String,
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

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn kind(&self) -> String {
    "Command".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(Command::new(self.token.clone(), self.name.clone()))
  }
}

#[derive(Clone)]
pub struct Assign {
  token: Token,
  pub left: Box<dyn AST>,
  pub right: Box<dyn AST>,
}

impl Assign {
  pub fn new(left: Box<dyn AST>, token: Token, right: Box<dyn AST>) -> Assign {
    Assign { left, token, right }
  }
}

impl std::fmt::Debug for Assign {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "({:#?} {} {:#?})",
      self.left, self.token.value, self.right
    )
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

  fn token(&self) -> Option<Token> {
    Some(self.token.clone())
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    Some(self.left.clone())
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    Some(self.right.clone())
  }

  fn kind(&self) -> String {
    "Assign".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(Assign::new(
      self.left.clone(),
      self.token.clone(),
      self.right.clone(),
    ))
  }
}

#[derive(Clone)]
pub struct Statement {
  pub equation: Box<Assign>,
  pub command: Box<Command>,
}

impl Statement {
  pub fn new<'a>(equation: Box<Assign>, command: Box<Command>) -> Statement {
    Statement { equation, command }
  }
}

impl std::fmt::Debug for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:#?}", self.equation)
  }
}

impl AST for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:#?}", self.equation)
  }

  fn token(&self) -> Option<Token> {
    None
  }

  fn left(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn right(&self) -> Option<Box<dyn AST>> {
    None
  }

  fn kind(&self) -> String {
    "Statement".to_string()
  }

  fn _clone(&self) -> Box<dyn AST> {
    Box::new(Statement::new(self.equation.clone(), self.command.clone()))
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

  fn factor(&mut self) -> Box<dyn AST> {
    let token = self.current_token.clone();
    if token.kind == TokenKind::Add {
      self.eat(TokenKind::Add);
      return Box::new(UnaryOp::new(token, self.factor()));
    } else if token.kind == TokenKind::Subtract {
      self.eat(TokenKind::Subtract);
      return Box::new(UnaryOp::new(token, self.factor()));
    } else if token.kind == TokenKind::Number {
      self.eat(TokenKind::Number);
      return Box::new(Number::new(token));
    } else if token.kind == TokenKind::Identifier {
      self.eat(TokenKind::Identifier);
      if self.current_token.kind == TokenKind::LeftParen {
        self.eat(TokenKind::LeftParen);
        let mut args = Vec::new();
        while self.current_token.kind != TokenKind::RightParen {
          args.push(self.expr());
          if self.current_token.kind == TokenKind::Comma {
            self.eat(TokenKind::Comma);
          }
        }
        self.eat(TokenKind::RightParen);
        return Box::new(Call::new(token, args));
      } else {
        return Box::new(Variable::new(token));
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

  fn term(&mut self) -> Box<dyn AST> {
    let mut node = self.factor();

    while [
      TokenKind::Multiply,
      TokenKind::Divide,
      TokenKind::LeftParen,
      TokenKind::Variable,
      TokenKind::Number,
    ]
    .contains(&self.current_token.kind)
    {
      let mut token = self.current_token.clone();
      if token.kind == TokenKind::Multiply {
        self.eat(TokenKind::Multiply);
      } else if token.kind == TokenKind::Divide {
        self.eat(TokenKind::Divide);
      } else if [TokenKind::LeftParen, TokenKind::Variable, TokenKind::Number]
        .contains(&self.current_token.kind)
      {
        token = Token::new(TokenKind::Multiply, &"", token.position.clone());
      }
      node = Box::new(BinOp::new(node, token, self.factor()));
    }

    node
  }

  fn indices(&mut self) -> Box<dyn AST> {
    let mut node = self.term();

    while [TokenKind::Power].contains(&self.current_token.kind) {
      let token = self.current_token.clone();
      if token.kind == TokenKind::Power {
        self.eat(TokenKind::Power);
      }
      node = Box::new(BinOp::new(node, token, self.term()));
    }

    node
  }

  fn expr(&mut self) -> Box<dyn AST> {
    if self.current_token.kind == TokenKind::Semicolon {
      self.error(format!(
        "SyntaxError: Unexpected {:?}: '{}' at position {}:{}",
        self.current_token.kind.clone(),
        self.current_token.value.clone(),
        self.current_token.position.human.line.clone(),
        self.current_token.position.human.column.clone(),
      ));
    }
    let mut node = self.indices();
    while [TokenKind::Add, TokenKind::Subtract].contains(&self.current_token.kind) {
      let token = self.current_token.clone();
      if token.kind == TokenKind::Add {
        self.eat(TokenKind::Add);
      } else if token.kind == TokenKind::Subtract {
        self.eat(TokenKind::Subtract);
      }
      node = Box::new(BinOp::new(node, token, self.term()));
    }

    node
  }

  fn command(&mut self) -> Box<Command> {
    let token = self.current_token.clone();
    if token.kind == TokenKind::Command {
      self.eat(TokenKind::Command);
      let token_id = self.current_token.clone();
      self.eat(TokenKind::Identifier);
      return Box::new(Command::new(token.clone(), token_id.value.to_string()));
    }
    self.error(format!(
      "SyntaxError: Unexpected {:?}: '{}' at position {}:{} expecting {:?}",
      self.current_token.kind.clone(),
      self.current_token.value.clone(),
      self.current_token.position.human.line.clone(),
      self.current_token.position.human.column.clone(),
      TokenKind::Command,
    ));
  }

  fn assign(&mut self) -> Box<Assign> {
    let left = self.expr();
    let token = self.current_token.clone();
    self.eat(TokenKind::Assign);
    let right = self.expr();
    Box::new(Assign::new(left, token, right))
  }

  fn statement(&mut self) -> Box<Statement> {
    let node = self.assign();
    self.eat(TokenKind::Semicolon);
    Box::new(Statement::new(node, self.command()))
  }

  fn statements(&mut self) -> Vec<Box<Statement>> {
    let mut statements = Vec::new();
    while self.current_token.kind != TokenKind::EOF {
      statements.push(self.statement());
    }
    statements
  }

  pub fn parse(&mut self) -> Vec<Box<Statement>> {
    self.statements()
  }
}
