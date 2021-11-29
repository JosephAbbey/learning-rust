use super::lexer::{Lexer, Token};

pub struct Parser {
  lexer: Lexer,
  current_token: Token,
  peek_token: Token,
}
