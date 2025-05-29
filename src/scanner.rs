use std::cmp::Ordering;
use std::collections::HashMap;

use std::vec;

use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token>{
        while !(self.is_at_end()) {
            // after the last self.scan_token(),
            //the self.current will point the next token's first char
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: Literal::None,
            line: self.line,
        });

        &self.tokens
    }
    fn scan_token(&mut self) {
        // comsume a char
        let token = self.source.as_bytes()[self.current] as char;
        self.current += 1;

        match token {
            // single length operators
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => self.add_token(TokenType::Dot, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Plus, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),

            // single or double length operators
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Bang, Literal::None);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Equal, Literal::None);
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Less, Literal::None);
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Greater, Literal::None);
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    // comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::None);
                }
            }

            // newlines and whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,

            // String literals
            '"' => self.add_string(),

            _ => {
                if token.is_ascii_digit() {
                    self.add_number();
                } else if token.is_alphabetic() || token == '_' {
                    self.add_identifier();
                } else {
                    eprintln!("{}: Unexpected character.", self.line);
                    self.has_error = true;
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
        });
    }

    fn add_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.is_at_end() {
            eprintln!("{}: Unterminated String.", self.line);
            self.has_error = true;
        }

        // at the past the while match the closing "
        self.current += 1;

        // Trim surrouding quotes
        // "hello world"
        // the first char is "
        // the last char is the "
        let value = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .unwrap()
            .to_string();
        self.add_token(TokenType::String, Literal::String(value));
    }

    fn add_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.current += 1;
        }

        // look for fractional part of number
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // consume the '.'
            self.current += 1;
        }

        while self.peek().is_ascii_digit() {
            self.current += 1;
        }

        let value: f64 = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .parse()
            .unwrap();
        self.add_token(TokenType::Number, Literal::Number(value));
    }

    fn add_identifier(&mut self) {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.current += 1;
        }

        let text = self.source.get(self.start..self.current).unwrap();
        match keywords.get(text) {
            Some(token_type) => self.add_token(token_type.clone(), Literal::None),
            None => self.add_token(TokenType::Identifier, Literal::None),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        let mut scanner = Scanner::new(String::from(
            "andy formLess fo _ _123 _abc ab123 \n abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
        ));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("andy"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("formLess"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("fo"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("_"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("_123"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("_abc"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("ab123"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from(
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                ),
                literal: Literal::None,
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                literal: Literal::None,
                line: 2,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i,token) in tokens.iter().enumerate(){
            assert_eq!(*token,expected_tokens[i]);
        }
        // assert_eq!(tokens.len(), expected_tokens.len());
        // for (i, token) in tokens.iter().enumerate() {
        //     assert_eq!(*token, expected_tokens[i]);
        // }
    }
}
