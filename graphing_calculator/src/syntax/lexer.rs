#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind {
  // Single-character tokens.
  LeftParen,
  RightParen,
  Subtract,
  Add,
  Divide,
  Multiply,
  Assign,
  Comma,
  Semicolon,
  Command,

  // Literals.
  Identifier,
  Number,
  Variable,

  // Extras.
  EOF,
}

// struct to hold ths human readable components of a position in the source code
#[derive(PartialEq, Debug, Clone)]
struct HumanPosition {
  pub line: usize,
  pub column: usize,
}

// struct to hold a position in the source code
#[derive(PartialEq, Debug, Clone)]
pub struct Position {
  human: HumanPosition,
  machine: usize,
}

// struct to hold a token
#[derive(Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub value: String,
  pub position: Position,
}

impl std::fmt::Display for Token {
  // convert token to string for debugging
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Token {{ kind: TokenKind::{:?}, value: {}, pos: ({}, {}) }}",
      self.kind, self.value, self.position.human.line, self.position.human.column
    )
  }
}

impl Token {
  // instantiate a token
  pub fn new(kind: TokenKind, value: &dyn std::string::ToString, pos: Position) -> Token {
    Token {
      kind: kind,
      value: value.to_string(),
      position: pos,
    }
  }
}

// struct to hold information for a lexer
pub struct Lexer {
  input: String,
  position: Position,
  current_char: Option<char>,
}

impl Lexer {
  // instantiate a lexer
  pub fn new(input: String) -> Lexer {
    let mut lexer = Lexer {
      input: input,
      position: Position {
        human: HumanPosition { line: 1, column: 1 },
        machine: 0,
      },
      current_char: Some(' '),
    };
    lexer.advance();
    lexer
  }

  // method to call for lexing errors
  fn error(&self, msg: &str) -> ! {
    panic!("{}", msg)
  }

  // advance the lexer
  fn advance(&self) {
    self.position.machine += 1;
    self.position.human.column += 1;
    if self.position.machine > self.input.len() {
      self.current_char = None; // Indicates end of input
    } else {
      self.current_char = self.input[self.position.machine..].chars().next();
    }
  }

  // peek at the next character without advancing
  fn peek(&self) -> Option<char> {
    let peek_pos = self.position.machine + 1;
    if peek_pos > self.input.len() {
      return None;
    } else {
      return self.input[peek_pos..].chars().next();
    }
  }

  // peek at the next token without advancing
  fn peek_token(&self) -> Token {
    let position = self.position.clone();
    let token = self.get_next_token();
    self.position = position;
    token
  }

  // advance until the token in not a whitespace character
  fn skip_whitespace(&self) {
    while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
      if self.current_char.unwrap() == '\n' {
        self.position.human.line += 1;
        self.position.human.column = 1;
      }
      self.advance();
    }
  }

  // skip until the and of the line (used for comments)
  fn skip_comment(&self) {
    while self.current_char.is_some() && self.current_char.unwrap() != '\n' {
      self.advance();
    }
  }

  // get a number and advance to the end of it
  fn number(&self) -> String {
    let result = String::new();
    while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
      result += &self.current_char.unwrap().to_string();
      self.advance();
    }
    if self.current_char.is_some() && self.current_char.unwrap() == '.' {
      result += ".";
      self.advance();
      while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
        result += &self.current_char.unwrap().to_string();
        self.advance();
      }
    }
    result
  }

  // get the next token (the main method)
  fn get_next_token(&self) -> Token {
    while self.current_char.is_some() {
      // skip comments
      if self.current_char.unwrap() == '#' {
        self.skip_comment();
        continue;
      }

      // skip whitespace
      if self.current_char.unwrap().is_whitespace() {
        self.skip_whitespace();
        continue;
      }

      // get a number
      if self.current_char.unwrap().is_digit(10) {
        let number = self.number();
        return Token::new(
          TokenKind::Number,
          &number,
          Position {
            human: HumanPosition {
              line: self.position.human.line,
              column: self.position.human.column - number.to_string().len(),
            },
            machine: self.position.machine,
          },
        );
      }

      // get a variable
      if self.current_char.unwrap().is_alphabetic() {
        let token = Token::new(
          TokenKind::Variable,
          &self.current_char.unwrap(),
          Position {
            human: HumanPosition {
              line: self.position.human.line,
              column: self.position.human.column,
            },
            machine: self.position.machine,
          },
        );
        self.advance();
        return token;
      }

      // get an assignment
      if self.current_char.unwrap() == '=' {
        self.advance();
        return Token::new(TokenKind::Assign, &'=', self.position);
      }

      // get a semicolon
      if self.current_char.unwrap() == ';' {
        self.advance();
        return Token::new(TokenKind::Semicolon, &';', self.position);
      }

      // get a comma
      if self.current_char.unwrap() == ',' {
        self.advance();
        return Token::new(TokenKind::Comma, &',', self.position);
      }

      // get a plus
      if self.current_char.unwrap() == '+' {
        self.advance();
        return Token::new(TokenKind::Add, &'+', self.position);
      }

      // get a minus
      if self.current_char.unwrap() == '-' {
        self.advance();
        return Token::new(TokenKind::Subtract, &'-', self.position);
      }

      // get a multiply
      if self.current_char.unwrap() == '*' {
        self.advance();
        return Token::new(TokenKind::Multiply, &'*', self.position);
      }

      // get a divide
      if self.current_char.unwrap() == '/' {
        self.advance();
        return Token::new(TokenKind::Divide, &'/', self.position);
      }

      // get a left parenthesis
      if self.current_char.unwrap() == '(' {
        self.advance();
        return Token::new(TokenKind::LeftParen, &'(', self.position);
      }

      // get a right parenthesis
      if self.current_char.unwrap() == ')' {
        self.advance();
        return Token::new(TokenKind::RightParen, &')', self.position);
      }

      // if none of the above, panic
      self.error(format!(
        "SyntaxError: Unexpected '{}' at position {}:{}",
        self.current_char.unwrap_or(' '),
        self.position.human.line,
        self.position.human.column - 1
      ));
    }

    return Token::new(TokenKind::EOF, &"EOF", self.position);
  }
}
