use crate::token::Token;
use crate::token_type::TokenType;

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,

    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub col: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: vec![Token::new(
                TokenType::Begin,
                String::new(),
                String::new(),
                1,
                1,
            )],
            start: 0,
            current: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, String::from("(")),
            ')' => self.add_token(TokenType::RightParen, String::from(")")),
            '{' => self.add_token(TokenType::LeftBrace, String::from("{")),
            '}' => self.add_token(TokenType::RightBrace, String::from("}")),
            '[' => self.add_token(TokenType::LeftSquareBracket, String::from("[")),
            ']' => self.add_token(TokenType::RightSquareBracket, String::from("]")),
            ':' => self.add_token(TokenType::Colon, String::from(":")),
            ',' => self.add_token(TokenType::Comma, String::from(",")),
            '.' => self.add_token(TokenType::Dot, String::from(".")),
            '+' => self.add_token(TokenType::Plus, String::from("+")),
            ';' => self.add_token(TokenType::Semicolon, String::from(";")),
            '/' => self.add_token(TokenType::Slash, String::from("/")),
            '*' => self.add_token(TokenType::Star, String::from("*")),
            //
            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            // One or two character tokens
            '-' => {
                if self._match('>') {
                    self.add_token(TokenType::ThinArrow, String::from("->"))
                } else {
                    self.add_token(TokenType::Minus, String::from("-"))
                }
            }
            '!' => {
                if self._match('=') {
                    self.add_token(TokenType::BangEqual, String::from("!="))
                } else {
                    self.add_token(TokenType::Bang, String::from("!"))
                }
            }
            '=' => {
                if self._match('=') {
                    self.add_token(TokenType::EqualEqual, String::from("=="))
                } else {
                    self.add_token(TokenType::Equal, String::from("="))
                }
            }
            '<' => {
                if self._match('=') {
                    self.add_token(TokenType::LessEqual, String::from("<="))
                } else {
                    self.add_token(TokenType::Less, String::from("<"))
                }
            }
            '>' => {
                if self._match('=') {
                    self.add_token(TokenType::GreaterEqual, String::from(">="))
                } else {
                    self.add_token(TokenType::Greater, String::from(">"))
                }
            }
            ' ' | '\r' => self.col += self.current - self.start,
            '\t' => self.col += self.current - self.start + 4,
            '\n' => {
                self.line += 1;
                self.col = 1
            }
            '`' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                    panic!("Unexpected character at Ln {}, Col {}", self.line, self.col)
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let slice = &self.source[self.start..self.current];
        let text: String = slice.into_iter().collect();

        let token_type = match &text[..] {
            "and" => TokenType::And,
            "def" => TokenType::Def,
            "else" => TokenType::Else,
            "empty" => TokenType::Empty,
            "exit" => TokenType::Exit,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "init" => TokenType::Init,
            "let" => TokenType::Let,
            "next" => TokenType::Next,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "struct" => TokenType::Struct,
            "super" => TokenType::Super,
            "test" => TokenType::Test,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type, text);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let source = self.source.clone();
        let value = &source[self.start..self.current];
        self.add_token(TokenType::Number, value.into_iter().collect());
    }

    fn string(&mut self) {
        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.col = 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string.");
        }

        self.advance();

        let source = self.source.clone();
        let value = &source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, value.into_iter().collect());
    }

    fn _match(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source[self.current] != expected) {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }

        return self.source[self.current + 1];
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, _type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            _type,
            text.into_iter().collect(),
            literal,
            self.line,
            self.col,
        ));
        self.col += self.current - self.start;
    }
}
