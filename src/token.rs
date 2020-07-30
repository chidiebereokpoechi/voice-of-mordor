use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: usize, col: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            col,
        }
    }
}
