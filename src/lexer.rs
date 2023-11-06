use crate::token::{TokenType, Token};
use crate::helpers::*;

pub(crate) fn scan(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut current = 0;
    let mut line = 1;

    while current < source.len()  {
        let start = current;
        let c = source.as_bytes()[current];
        current += 1;

        match c {
            b'(' => { tokens.push(make_token(TokenType::LParen, source, start, current, line)); }
            b')' => { tokens.push(make_token(TokenType::RParen, source, start, current, line)); }
            b':' => { tokens.push(make_token(TokenType::Colon, source, start, current, line)); }
            b',' => { tokens.push(make_token(TokenType::Comma, source, start, current, line)); }
            b'+' => { tokens.push(make_token(TokenType::Plus, source, start, current, line)); }
            b'-' => { tokens.push(make_token(TokenType::Minus, source, start, current, line)); }
            b'*' => { tokens.push(make_token(TokenType::Star, source, start, current, line)); }
            b';' => { tokens.push(make_token(TokenType::Semicolon, source, start, current, line)); }
            b'\n' => {
                line += 1;
                tokens.push(make_token(TokenType::NewLine, source, start, current, line));
            }
            b'=' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(make_token(TokenType::Eq, source, start, current, line));
                } else {
                    tokens.push(make_token(TokenType::Assign, source, start, current, line));
                }
            }
            b'<' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(make_token(TokenType::LtEq, source, start, current, line));
                } else {
                    tokens.push(make_token(TokenType::Lt, source, start, current, line));
                }
            }
            b'>' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(make_token(TokenType::GtEq, source, start, current, line));
                } else {
                    tokens.push(make_token(TokenType::Gt, source, start, current, line));
                }
            }
            b'!' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(make_token(TokenType::BangEq, source, start, current, line));
                } else {
                    tokens.push(make_token(TokenType::Bang, source, start, current, line));
                }
            }
            b'/' => {
                if peek(source, current) == b'/' {
                    while peek(source, current) != b'\n' && current < source.len() { current += 1; }
                } else {
                    tokens.push(make_token(TokenType::Slash, source, start, current, line));
                }
            }
            b'"' => { tokens.push(string(source, start, &mut current, &mut line)); }
            b' ' | b'\t' | b'\r' => (),
            _ => {
                if is_alpha(c) {
                    tokens.push(ident(source, start, &mut current, line));
                } else if is_digit(c) {
                    tokens.push(num(source, start, &mut current, line));
                } else {
                    tokens.push(Token::new(TokenType::Illegal, "", line))
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, "", line));

    tokens
}

fn make_token<'a>(token_type: TokenType, source: &'a str, start: usize, current: usize, line: i32) -> Token<'a> {
    let lexeme= &source[start..current];
    Token::new(token_type, lexeme, line)
}

fn peek<'a>(source: &'a str, current: usize) -> u8 {
    if current >= source.len() { return b'\0'; }
    source.as_bytes()[current]
}

fn peek_next<'a>(source: &'a str, current: usize) -> u8 {
    if current + 1 >= source.len() { return b'\0'; }
    source.as_bytes()[current + 1]
}

fn ident<'a>(source: &'a str, start: usize, current: &mut usize, line: i32) -> Token<'a> {
    while is_alpha(peek(source, *current)) || is_digit(peek(source, *current)) {
        *current += 1;
    }

    lookup_keyword(source, start, *current, line)
}

fn lookup_keyword<'a>(source: &'a str, start: usize, current: usize, line: i32) -> Token<'a> {
    let c = source.as_bytes()[start];
    match c {
        b'l' => { return check_keyword(source, start, current, line, "et", 1, 2, TokenType::Let); }
        b'r' => { return check_keyword(source, start, current, line, "eturn", 1, 5, TokenType::Return); }
        b'i' => {
            if current - start > 1 {
                match source.as_bytes()[start + 1] {
                    b'f' => { return make_token(TokenType::If, source, start, current, line); }
                    b'n' => { return make_token(TokenType::In, source, start, current, line); }
                    _ => (),
                }
            }
        }
        b't' => { return check_keyword(source, start, current, line, "rue", 1, 3, TokenType::True); }
        b'p' => { return check_keyword(source, start, current, line, "rint", 1, 4, TokenType::Print); }
        b'e' => {
            if current - start > 1 {
                match source.as_bytes()[start + 1] {
                    b'n' => { return check_keyword(source, start, current, line, "d", 2, 1, TokenType::End); }
                    b'l' => { return check_keyword(source, start, current, line, "se", 2, 2, TokenType::Else); }
                    _ => (),
                }
            }
        }
        b'f' => {
            if current - start > 1 {
                match source.as_bytes()[start + 1] {
                    b'u' => { return check_keyword(source, start, current, line, "n", 2, 1, TokenType::Fun); }
                    b'a' => { return check_keyword(source, start, current, line, "lse", 2, 3, TokenType::False); }
                    b'o' => { return check_keyword(source, start, current, line, "r", 2, 1, TokenType::For); }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    make_token(TokenType::Ident, source, start, current, line)
}

fn check_keyword<'a>(source: &'a str, start: usize, current: usize, line: i32, rest: &str, begin: usize, len: usize, token_type: TokenType) -> Token<'a> {
    if current - start == begin + len {
        let slice = &source[start+begin..start+begin+len];
        if rest == slice {
            return make_token(token_type, source, start, current, line);
        }
    }
    make_token(TokenType::Ident, source, start, current, line)
}

fn num<'a>(source: &'a str, start: usize, current: &mut usize, line: i32) -> Token<'a> {
    while is_digit(peek(source, *current)) { *current += 1; }
    if peek(source, *current) == b'.' && is_digit(peek_next(source, *current)) {
        *current += 1;

        while is_digit(peek(source, *current)) { *current += 1; }
    }

    make_token(TokenType::Num, source, start, *current, line)
}

fn string<'a>(source: &'a str, start: usize, current: &mut usize, line: &mut i32) -> Token<'a> {
    while peek(source, *current) != b'"' && *current < source.len() {
        *current += 1;

        if peek(source, *current) == b'\n' { *line += 1; }
    }

    if *current >= source.len() {
        panic!("unterminated string");
    }

    *current += 1;
    let lexeme = &source[start + 1..*current - 1];
    Token::new(TokenType::String, lexeme, *line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_illegal() {
        let source = "👿";
        let tokens = scan(source);

        let token = &tokens.first().unwrap().token_type;
        assert_eq!(*token, TokenType::Illegal);
    }

    #[test]
    fn test_next_token() {
        let source = r#"let num1 = 5
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
            print "free
            numba 9"
        end
        "#;

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
                lexeme: "free\n            numba 9",
                line: 23,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 24,
            },
            Token {
                token_type: TokenType::End,
                lexeme: "end",
                line: 24,
            },
            Token {
                token_type: TokenType::NewLine,
                lexeme: "\n",
                line: 25,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "",
                line: 25,
            },
        ];

        let tokens = scan(source);
        for (i, e) in expected.iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e.token_type, t.token_type, "error position {} expected == {:?} got == {:?}", i, e, t);
            assert_eq!(e.lexeme, t.lexeme, "error position {} expected == {:?} got == {:?}", i, e, t);
            assert_eq!(e.line, t.line, "error position {} expected == {:?} got == {:?}", i, e, t)
        }
    }
}
