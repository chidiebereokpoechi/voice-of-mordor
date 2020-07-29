use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquareBracket,
    RightSquareBracket,
    Colon,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    ThinArrow,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Def,
    Else,
    Empty,
    Exit,
    False,
    For,
    If,
    Init,
    Let,
    Next,
    Or,
    Print,
    Return,
    Struct,
    Super,
    Test,
    This,
    True,
    While,

    Begin,
    End,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = format!("{:?}", self);
        write!(f, "{}", text)
    }
}
