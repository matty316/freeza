use crate::token::*;
use std::collections::HashMap;

pub(crate)struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
    line: u32,
}

impl Lexer {
    pub(crate)fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: b'\0',
            line: 1,
        };

        l.read_char();
        return l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(& mut self) {
        while self.ch == b' ' || self.ch == b'\r' || self.ch == b'\t' {
            self.read_char()
        }
    }

    fn read_ident(&mut self) -> Token {
        
        let start = self.position;

        while is_alphanumeric(self.ch) {
            self.read_char();
        }

        let identifier = &self.input[start..self.position];

        let keywords = HashMap::from([
            ("let", TokenType::Let),
            ("fun", TokenType::Fun),
            ("return", TokenType::Return),
            ("end", TokenType::End),
            ("if", TokenType::If),
            ("else", TokenType::Else),
        ]);

        match keywords.get(identifier) {
            Some(t) => return Token {token_type: t.clone(), literal: identifier.to_string(), line: self.line},
            _ => return Token {token_type: TokenType::Ident, literal: identifier.to_string(), line: self.line},
        }
    }

    fn read_num(&mut self) -> Token {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        if self.ch == b'.' && is_digit(self.peek()) {
            self.read_char();
            while is_digit(self.ch) {
                self.read_char();
            }
        }

        let n = &self.input[start..self.position];

        if n.as_bytes().contains(&b'.') {
            Token {token_type: TokenType::Float, literal: n.to_string(), line: self.line}
        } else {
            Token {token_type: TokenType::Int, literal: n.to_string(), line: self.line}
        }
    }

    fn read_string(&mut self) -> Token {
        self.read_char();
        let start = self.position;

        while self.peek() != b'"' {
            self.read_char()
        }
        self.read_char();

        let s = &self.input[start..self.position];

        return Token{token_type: TokenType::String, literal: s.to_string(), line: self.line}
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return b'\0';
        } 
        return self.input.as_bytes()[self.read_position]
    }

    fn new_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            literal: "".to_string(),
            line: self.line
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        match self.ch {
            b';' => tok = self.new_token(TokenType::Semicolon),
            b'(' => tok = self.new_token(TokenType::LParen),
            b')' => tok = self.new_token(TokenType::RParen),
            b',' => tok = self.new_token(TokenType::Comma),
            b':' => tok = self.new_token(TokenType::Colon),
            b'+' => tok = self.new_token(TokenType::Plus),
            b'-' => tok = self.new_token(TokenType::Minus),
            b'*' => tok = self.new_token(TokenType::Star),
            b'/' => tok = self.new_token(TokenType::Slash),
            b'<' => {
                if self.peek() == b'=' {
                    self.read_char();
                    tok = self.new_token(TokenType::LtEq);
                } else {
                    tok = self.new_token(TokenType::Lt);
                }
            },
            b'>' => {
                if self.peek() == b'=' {
                    self.read_char();
                    tok = self.new_token(TokenType::GtEq);
                } else {
                    tok = self.new_token(TokenType::Gt);
                }
            },
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    tok = self.new_token(TokenType::Eq);
                } else {
                    tok = self.new_token(TokenType::Assign);
                }
            },
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    tok = self.new_token(TokenType::BangEq);
                } else {
                    tok = self.new_token(TokenType::Bang);
                }
            },
            b'"' => {
                tok = self.read_string();
                self.read_char();
                return tok;
            }
            b'\n' => {
                tok = self.new_token(TokenType::NewLine);
                self.line += 1;
            },
            b'\0' => tok = self.new_token(TokenType::Eof),
            _ => {
                if is_alpha(self.ch) {
                    tok = self.read_ident();
                    return tok;
                } else if is_digit(self.ch) {
                    tok = self.read_num();
                    return tok;
                } else {
                    tok = Token {token_type: TokenType::Illegal, literal: "".to_string(), line: self.line}
                }
            },
        }

        self.read_char();
        tok
    }
}



fn is_alpha(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

fn is_alphanumeric(ch: u8) -> bool {
    is_alpha(ch) || is_digit(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r###"let num1 = 5
        let num2 = 10.5

        let string = "name"; let anotherString = "another name"

        fun add(x, y):
            return x + y   
        end

        let result = add(five, ten)
        !-/*<> <= >= == !=

        if 5 < 10:
            return "if"
        else if 6 > 5:
            return "else if"
        else:
            return "else"
        end

        "###;

        let expected = vec![
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "num1".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Assign,
                literal: "".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "num2".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Assign,
                literal: "".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Float,
                literal: "10.5".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "string".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::Assign,
                literal: "".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::String,
                literal: "name".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: "".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "anotherString".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::Assign,
                literal: "".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::String,
                literal: "another name".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 5,
            },
            Token {
                token_type: TokenType::Fun,
                literal: "fun".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "add".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::LParen,
                literal: "".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "x".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::Comma,
                literal: "".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::RParen,
                literal: "".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::Colon,
                literal: "".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 6,
            },
            Token {
                token_type: TokenType::Return,
                literal: "return".to_string(),
                line: 7,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "x".to_string(),
                line: 7,
            },
            Token {
                token_type: TokenType::Plus,
                literal: "".to_string(),
                line: 7,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
                line: 7,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 7,
            },
            Token {
                token_type: TokenType::End,
                literal: "end".to_string(),
                line: 8,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 8,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 9,
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "result".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Assign,
                literal: "".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "add".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::LParen,
                literal: "".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "five".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Comma,
                literal: "".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                literal: "ten".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::RParen,
                literal: "".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 10,
            },
            Token {
                token_type: TokenType::Bang,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::Minus,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::Slash,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::Star,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::Lt,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::Gt,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::LtEq,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::GtEq,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::Eq,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::BangEq,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 11,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 12,
            },
            Token {
                token_type: TokenType::If,
                literal: "if".to_string(),
                line: 13,
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
                line: 13,
            },
            Token {
                token_type: TokenType::Lt,
                literal: "".to_string(),
                line: 13,
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
                line: 13,
            },
            Token {
                token_type: TokenType::Colon,
                literal: "".to_string(),
                line: 13,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 13,
            },
            Token {
                token_type: TokenType::Return,
                literal: "return".to_string(),
                line: 14,
            },
            Token {
                token_type: TokenType::String,
                literal: "if".to_string(),
                line: 14,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 14,
            },
            Token {
                token_type: TokenType::Else,
                literal: "else".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::If,
                literal: "if".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::Int,
                literal: "6".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::Gt,
                literal: "".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::Colon,
                literal: "".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 15,
            },
            Token {
                token_type: TokenType::Return,
                literal: "return".to_string(),
                line: 16,
            },
            Token {
                token_type: TokenType::String,
                literal: "else if".to_string(),
                line: 16,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 16,
            },
            Token {
                token_type: TokenType::Else,
                literal: "else".to_string(),
                line: 17,
            },
            Token {
                token_type: TokenType::Colon,
                literal: "".to_string(),
                line: 17,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 17,
            },
            Token {
                token_type: TokenType::Return,
                literal: "return".to_string(),
                line: 18,
            },
            Token {
                token_type: TokenType::String,
                literal: "else".to_string(),
                line: 18,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 18,
            },
            Token {
                token_type: TokenType::End,
                literal: "end".to_string(),
                line: 19,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 19,
            },
            Token {
                token_type: TokenType::NewLine,
                literal: "".to_string(),
                line: 20,
            },
            Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
                line: 21,
            },
        ];

        let mut l = Lexer::new(input.to_string());

        for (i, e) in expected.iter().enumerate() {
            let t = l.next_token();
            assert_eq!(e.token_type, t.token_type, "error position {} expected == {:?} got == {:?}", i, e, t);
            assert_eq!(e.literal, t.literal);
            assert_eq!(e.line, t.line)
        }
    }
}
