use crate::token::{TokenType, Token};
use crate::helpers::*;

pub(crate) struct Lexer<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Lexer {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        let c = self.advance();
        if is_alpha(c) { return self.ident(); }
        if is_digit(c) { return self.number(); }

        match c {
            b'(' => return self.make_token(TokenType::LParen),
            b')' => return self.make_token(TokenType::RParen),
            b':' => return self.make_token(TokenType::Colon),
            b',' => return self.make_token(TokenType::Comma),
            b'+' => return self.make_token(TokenType::Plus),
            b'-' => return self.make_token(TokenType::Minus),
            b'*' => return self.make_token(TokenType::Star),
            b';' => return self.make_token(TokenType::Semicolon),
            b'\n' => {
                self.line += 1;
                return self.make_token(TokenType::NewLine);
            }
            b'=' => {
                if self.peek() == b'=' {
                    self.advance();
                    return self.make_token(TokenType::Eq);
                } else {
                    return self.make_token(TokenType::Assign);
                }
            }
            b'<' => {
                if self.peek() == b'=' {
                    self.advance();
                    return self.make_token(TokenType::LtEq);
                } else {
                    return self.make_token(TokenType::Lt);
                }
            }
            b'>' => {
                if self.peek() == b'=' {
                    self.advance();
                    return self.make_token(TokenType::GtEq);
                } else {
                    return self.make_token(TokenType::Gt);
                }
            }
            b'!' => {
                if self.peek() == b'=' {
                    self.advance();
                    return self.make_token(TokenType::BangEq);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            }
            b'/' => {
                if self.peek() == b'/' {
                    while self.peek() != b'\n' { self.advance(); }
                } else {
                    return self.make_token(TokenType::Slash);
                }
            }
            b'"' => return self.string(),
            _ => (),
        }

        if self.is_at_end() { return self.make_token(TokenType::Eof); }

        self.make_token(TokenType::Illegal)
    }

    //Helpers
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                b' ' | b'\t' | b'\r' => { self.advance(); }
                _ => return,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        if self.is_at_end() { return b'\0' }
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lit = std::str::from_utf8(&self.source[self.start..self.current]);
        Token {
            token_type,
            lexeme: lit.expect("cannot get literal val"),
            line: self.line
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() { return b'\0' }
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() { return b'\0' }
        self.source[self.current + 1]
    }

    fn ident(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        self.lookup_keyword()
    }

    fn lookup_keyword(&self) -> Token {
        let c = &self.source[self.start];
        match c {
            b'l' => { return self.check_keyword("et", 1, 2, TokenType::Let); }
            b'r' => { return self.check_keyword("eturn", 1, 5, TokenType::Return); }
            b'i' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        b'f' => { return self.make_token(TokenType::If); }
                        b'n' => { return self.make_token(TokenType::In); }
                        _ => (),
                    }
                }
            }
            b't' => { return self.check_keyword("rue", 1, 3, TokenType::True); }
            b'p' => { return self.check_keyword("rint", 1, 4, TokenType::Print); }
            b'e' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        b'n' => { return self.check_keyword("d", 2, 1, TokenType::End); }
                        b'l' => { return self.check_keyword("se", 2, 2, TokenType::Else); }
                        _ => (),
                    }
                }
            }
            b'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        b'u' => { return self.check_keyword("n", 2, 1, TokenType::Fun); }
                        b'a' => { return self.check_keyword("lse", 2, 3, TokenType::False); }
                        b'o' => { return self.check_keyword("r", 2, 1, TokenType::For); }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        self.make_token(TokenType::Ident)
    }

    fn check_keyword(&self, rest: &str, start: usize, len: usize, token_type: TokenType) -> Token {
        if self.current - self.start == start + len {
            let slice = &self.source[self.start+start..self.start+start+len];
            if rest.as_bytes() == slice {
                return self.make_token(token_type);
            }
        }
        self.make_token(TokenType::Ident)
    }

    fn number(&mut self) -> Token {
        while is_digit(self.peek()) { self.advance(); }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) { self.advance(); }
        }

        self.make_token(TokenType::Num)
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' {
            self.advance();
            if self.peek() == b'\n' { self.line += 1; }
        }

        self.advance();
        let string = std::str::from_utf8(&self.source[self.start+1..self.current-1]).expect("cannot get string");
        Token {token_type: TokenType::String, lexeme: string, line: self.line}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"let num1 = 5
        let num2 = 10.5

        let string = "name"; let anotherString = "another name"

        fun add(x, y):
            return x + y   
        end

        let result = add(five, ten)
        !-/*<> <= >= == !=

        if true:
            return "if"
        else if false:
            return "else if"
        else:
            return "else"
        end

        for i in array:
            print "free numba 9"
        end
        "#;

        let mut l = Lexer::new(input);

        let expected = vec![
            Token {
                token_type: TokenType::Let,
                lexeme: "let",
                line: 1,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "num1",
                line: 1,
            },
            Token {
                token_type: TokenType::Assign,
                lexeme: "=",
                line: 1,
            },
            Token {
                token_type: TokenType::Num,
                lexeme: "5",
                line: 1,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 2,
            },
            Token {
                token_type: TokenType::Let,
                lexeme: "let",
                line: 2,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "num2",
                line: 2,
            },
            Token {
                token_type: TokenType::Assign,
                lexeme: "=",
                line: 2,
            },
            Token {
                token_type: TokenType::Num,
                lexeme: "10.5",
                line: 2,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 3,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 4,
            },
            Token {
                token_type: TokenType::Let,
                lexeme: "let",
                line: 4,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "string",
                line: 4,
            },
            Token {
                token_type: TokenType::Assign,
                lexeme: "=",
                line: 4,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "name",
                line: 4,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";",
                line: 4,
            },
            Token {
                token_type: TokenType::Let,
                lexeme: "let",
                line: 4,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "anotherString",
                line: 4,
            },
            Token {
                token_type: TokenType::Assign,
                lexeme: "=",
                line: 4,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "another name",
                line: 4,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 5,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 6,
            },
            Token {
                token_type: TokenType::Fun,
                lexeme: "fun",
                line: 6,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "add",
                line: 6,
            },
            Token {
                token_type: TokenType::LParen,
                lexeme: "(",
                line: 6,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "x",
                line: 6,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",",
                line: 6,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "y",
                line: 6,
            },
            Token {
                token_type: TokenType::RParen,
                lexeme: ")",
                line: 6,
            },
            Token {
                token_type: TokenType::Colon,
                lexeme: ":",
                line: 6,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 7,
            },
            Token {
                token_type: TokenType::Return,
                lexeme: "return",
                line: 7,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "x",
                line: 7,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+",
                line: 7,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "y",
                line: 7,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 8,
            },
            Token {
                token_type: TokenType::End,
                lexeme: "end",
                line: 8,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 9,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 10,
            },
            Token {
                token_type: TokenType::Let,
                lexeme: "let",
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "result",
                line: 10,
            },
            Token {
                token_type: TokenType::Assign,
                lexeme: "=",
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "add",
                line: 10,
            },
            Token {
                token_type: TokenType::LParen,
                lexeme: "(",
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "five",
                line: 10,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",",
                line: 10,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "ten",
                line: 10,
            },
            Token {
                token_type: TokenType::RParen,
                lexeme: ")",
                line: 10,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 11,
            },
            Token {
                token_type: TokenType::Bang,
                lexeme: "!",
                line: 11,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-",
                line: 11,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: "/",
                line: 11,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*",
                line: 11,
            },
            Token {
                token_type: TokenType::Lt,
                lexeme: "<",
                line: 11,
            },
            Token {
                token_type: TokenType::Gt,
                lexeme: ">",
                line: 11,
            },
            Token {
                token_type: TokenType::LtEq,
                lexeme: "<=",
                line: 11,
            },
            Token {
                token_type: TokenType::GtEq,
                lexeme: ">=",
                line: 11,
            },
            Token {
                token_type: TokenType::Eq,
                lexeme: "==",
                line: 11,
            },
            Token {
                token_type: TokenType::BangEq,
                lexeme: "!=",
                line: 11,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 12,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 13,
            },
            Token {
                token_type: TokenType::If,
                lexeme: "if",
                line: 13,
            },
            Token {
                token_type: TokenType::True,
                lexeme: "true",
                line: 13,
            },
            Token {
                token_type: TokenType::Colon,
                lexeme: ":",
                line: 13,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 14,
            },
            Token {
                token_type: TokenType::Return,
                lexeme: "return",
                line: 14,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "if",
                line: 14,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 15,
            },
            Token {
                token_type: TokenType::Else,
                lexeme: "else",
                line: 15,
            },
            Token {
                token_type: TokenType::If,
                lexeme: "if",
                line: 15,
            },
            Token {
                token_type: TokenType::False,
                lexeme: "false",
                line: 15,
            },
            Token {
                token_type: TokenType::Colon,
                lexeme: ":",
                line: 15,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 16,
            },
            Token {
                token_type: TokenType::Return,
                lexeme: "return",
                line: 16,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "else if",
                line: 16,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 17,
            },
            Token {
                token_type: TokenType::Else,
                lexeme: "else",
                line: 17,
            },
            Token {
                token_type: TokenType::Colon,
                lexeme: ":",
                line: 17,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 18,
            },
            Token {
                token_type: TokenType::Return,
                lexeme: "return",
                line: 18,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "else",
                line: 18,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 19,
            },
            Token {
                token_type: TokenType::End,
                lexeme: "end",
                line: 19,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 20,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 21,
            },
            Token {
                token_type: TokenType::For,
                lexeme: "for",
                line: 21,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "i",
                line: 21,
            },
            Token {
                token_type: TokenType::In,
                lexeme: "in",
                line: 21,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "array",
                line: 21,
            },
            Token {
                token_type: TokenType::Colon,
                lexeme: ":",
                line: 21,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 22,
            },
            Token {
                token_type: TokenType::Print,
                lexeme: "print",
                line: 22,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "free numba 9",
                line: 22,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 23,
            },
            Token {
                token_type: TokenType::End,
                lexeme: "end",
                line: 23,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 24,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "",
                line: 24,
            },
        ];
        
        for (i, e) in expected.iter().enumerate() {
            let t = l.next_token();
            assert_eq!(e.token_type, t.token_type, "error position {} expected == {:?} got == {:?}", i, e, t);
            assert_eq!(e.lexeme, t.lexeme, "error position {} expected == {:?} got == {:?}", i, e, t);
            assert_eq!(e.line, t.line, "error position {} expected == {:?} got == {:?}", i, e, t)
        }
    }
}
