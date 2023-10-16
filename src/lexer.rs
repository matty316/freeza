use crate::token::*;
use std::collections::HashMap;

pub(crate)struct Lexer {
    input: String
}

impl Lexer {
    pub(crate)fn new(input: String) -> Self {
        let l = Lexer {
            input: input
        };

        return l
    }

    fn is_at_end(&self, position: usize) -> bool {
        return position >= self.input.len();
    }

    fn read_char(&self, ch: &mut u8, position: &mut usize, read_position: &mut usize) {
        if *read_position >= self.input.len() {
            *ch = b'\0';
        } else {
            *ch = self.input.as_bytes()[*read_position];
        }
        *position = *read_position;
        *read_position += 1;
    }

    fn skip_whitespace(&self, ch: &mut u8, position: &mut usize, read_position: &mut usize) {
        while *ch == b' ' || *ch == b'\t' || *ch == b'\r' {
            self.read_char(ch, position, read_position)
        }
    }

    pub(crate)fn next_token(&self, ch: &mut u8, position: &mut usize, read_position: &mut usize, line: &mut u32) -> Token {
        let tok: Token;

        self.skip_whitespace(ch, position, read_position);

        match ch {
            b';' => tok = self.new_token(TokenType::Semicolon, *line),
            b'(' => tok = self.new_token(TokenType::LParen, *line),
            b')' => tok = self.new_token(TokenType::RParen, *line),
            b',' => tok = self.new_token(TokenType::Comma, *line),
            b':' => tok = self.new_token(TokenType::Colon, *line),
            b'+' => tok = self.new_token(TokenType::Plus, *line),
            b'-' => tok = self.new_token(TokenType::Minus, *line),
            b'*' => tok = self.new_token(TokenType::Star, *line),
            b'/' => tok = self.new_token(TokenType::Slash, *line),
            b'<' => {
                if self.peek(*position) == b'=' {
                    self.read_char(ch, position, read_position);
                    tok = self.new_token(TokenType::LtEq, *line);
                } else {
                    tok = self.new_token(TokenType::Lt, *line);
                }
            },
            b'>' => {
                if self.peek(*position) == b'=' {
                    self.read_char(ch, position, read_position);
                    tok = self.new_token(TokenType::GtEq, *line);
                } else {
                    tok = self.new_token(TokenType::Gt, *line);
                }
            },
            b'=' => {
                if self.peek(*position) == b'=' {
                    self.read_char(ch, position, read_position);
                    tok = self.new_token(TokenType::Eq, *line);
                } else {
                    tok = self.new_token(TokenType::Assign, *line);
                }
            },
            b'!' => {
                if self.peek(*position) == b'=' {
                    self.read_char(ch, position, read_position);
                    tok = self.new_token(TokenType::BangEq, *line);
                } else {
                    tok = self.new_token(TokenType::Bang, *line);
                }
            },
            b'"' => {
                tok = self.read_string(ch, position, read_position, *line);
            }
            b'\n' => {
                tok = self.new_token(TokenType::NewLine, *line);
                *line += 1;
            },
            b'\0' => tok = self.new_token(TokenType::Eof, *line),
            _ => {
                if is_alpha(*ch) {
                    tok = self.read_ident(ch, position, read_position, *line);
                    return tok;
                } else if is_digit(*ch) {
                    tok = self.read_num(ch, position, read_position, *line);
                    return tok;
                } else {
                    tok = self.new_token(TokenType::Illegal, *line);
                }
            },
        }
        self.read_char(ch, position, read_position);
        return tok;
    }

    fn read_ident(&self, ch: &mut u8, position: &mut usize, read_position: &mut usize, line: u32) -> Token {
        let start = *position;
        while is_alphanumeric(*ch) {
            self.read_char(ch, position, read_position);
        }

        let identifier = &self.input[start..*position];

        let keywords = HashMap::from([
            ("let", TokenType::Let),
            ("fun", TokenType::Fun),
            ("return", TokenType::Return),
            ("end", TokenType::End),
            ("if", TokenType::If),
            ("else", TokenType::Else),
        ]);

        match keywords.get(identifier) {
            Some(t) => return Token {token_type: t.clone(), literal: identifier.to_string(), line: line},
            _ => return Token {token_type: TokenType::Ident, literal: identifier.to_string(), line: line},
        }
    }

    fn read_num(&self, ch: &mut u8, position: &mut usize, read_position: &mut usize, line: u32) -> Token {
        let start = *position;
        while is_digit(*ch) {
            self.read_char(ch, position, read_position);
        }

        if *ch == b'.' && is_digit(self.peek(*position)) {
            self.read_char(ch, position, read_position);
            while is_digit(*ch) {
                self.read_char(ch, position, read_position);
            }
        }

        let n = &self.input[start..*position];

        if n.as_bytes().contains(&b'.') {
            Token {token_type: TokenType::Float, literal: n.to_string(), line: line}
        } else {
            Token {token_type: TokenType::Int, literal: n.to_string(), line: line}
        }
    }

    fn read_string(&self, ch: &mut u8, position: &mut usize, read_position: &mut usize, line: u32) -> Token {
        self.read_char(ch, position, read_position);

        let start = *position;

        while self.peek(*position) != b'"' {
            self.read_char(ch, position, read_position);
        }
        self.read_char(ch, position, read_position);

        let s = &self.input[start..*position];

        Token{token_type: TokenType::String, literal: s.to_string(), line: line}
    }

    fn peek(&self, position: usize) -> u8 {
        if position + 1 >= self.input.len() {
            return b'\0';
        } 
        self.input.as_bytes()[position+1]
    }

    fn new_token(&self, token_type: TokenType, line: u32) -> Token {
        Token {
            token_type: token_type,
            literal: "".to_string(),
            line: line,
        }
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

        let l = Lexer::new(input.to_string());

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
        
        let mut ch = input.as_bytes()[0];
        let mut position = 0;
        let mut read_position: usize = 1;
        let mut line = 1;
        
        for (i, e) in expected.iter().enumerate() {
            let t = l.next_token(&mut ch, &mut position, &mut read_position, &mut line);
            assert_eq!(e.token_type, t.token_type, "error position {} expected == {:?} got == {:?}", i, e, t);
            assert_eq!(e.literal, t.literal);
            assert_eq!(e.line, t.line)
        }
    }
}
