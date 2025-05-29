use crate::token_type;

use crate::TokenType;
struct Scanner {
    source: String,
    tokens: vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }
    fn scan_tokens(&mut self) {
        while (!(self.is_at_end())) {
            start = current;
            //
        }
    }
    fn scan_token(&mut self) {
        let token = self.source.as_bytes[self.current] as char;
        self.current += 1;

        match token {
            // single length operators
            '(' => self.add_token(TokenType::LEFT_PAREN, Literal::None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, Literal::None),
            '{' => self.add_token(TokenType::LEFT_BRACE, Literal::None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, Literal::None),
            ',' => self.add_token(TokenType::COMMA, Literal::None),
            '.' => self.add_token(TokenType::DOT, Literal::None),
            '-' => self.add_token(TokenType::MINUS, Literal::None),
            '+' => self.add_token(TokenType::PLUS, Literal::None),
            ';' => self.add_token(TokenType::SEMICOLON, Literal::None),
            '*' => self.add_token(TokenType::STAR, Literal::None),

            // single or double length operators
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BANG_EQUAL, Literal::None);
                } else {
                    self.add_token(TokenType::BANG, Literal::None);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, Literal::None);
                } else {
                    self.add_token(TokenType::EQUAL, Literal::None);
                }
            '<' => {
                if self.match_next_char('=')
            }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        })
    }

    // be used in match the second char in "!=", "==" and so on
    fn match_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn is_at_end(&self) -> bool {
        match self.current.cmp(&(self.source.len())) {
            Ordering::Less => false,
            _ => true,
        }
    }
}
