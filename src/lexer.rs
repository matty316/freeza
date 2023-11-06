use crate::token::Token;
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
            b'(' => { tokens.push(Token::LParen(line)); }
            b')' => { tokens.push(Token::RParen(line)); }
            b':' => { tokens.push(Token::Colon(line)); }
            b',' => { tokens.push(Token::Comma(line)); }
            b'+' => { tokens.push(Token::Plus(line)); }
            b'-' => { tokens.push(Token::Minus(line)); }
            b'*' => { tokens.push(Token::Star(line)); }
            b';' => { tokens.push(Token::Semicolon(line)); }
            b'\n' => {
                line += 1;
                tokens.push(Token::NewLine(line));
            }
            b'=' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(Token::Eq(line));
                } else {
                    tokens.push(Token::Assign(line));
                }
            }
            b'<' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(Token::LtEq(line));
                } else {
                    tokens.push(Token::Lt(line));
                }
            }
            b'>' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(Token::GtEq(line));
                } else {
                    tokens.push(Token::Gt(line));
                }
            }
            b'!' => {
                if peek(source, current) == b'=' {
                    current += 1;
                    tokens.push(Token::BangEq(line));
                } else {
                    tokens.push(Token::Bang(line));
                }
            }
            b'/' => {
                if peek(source, current) == b'/' {
                    while peek(source, current) != b'\n' && current < source.len() { current += 1; }
                } else {
                    tokens.push(Token::Slash(line));
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
                    tokens.push(Token::Illegal(line))
                }
            }
        }
    }

    tokens.push(Token::Eof);

    tokens
}

fn peek<'a>(source: &'a str, current: usize) -> u8 {
    if current >= source.len() { return b'\0'; }
    source.as_bytes()[current]
}

fn peek_next<'a>(source: &'a str, current: usize) -> u8 {
    if current + 1 >= source.len() { return b'\0'; }
    source.as_bytes()[current + 1]
}

fn ident<'a>(source: &'a str, start: usize, current: &mut usize, line: i32) -> Token {
    while is_alpha(peek(source, *current)) || is_digit(peek(source, *current)) {
        *current += 1;
    }

    lookup_keyword(source, start, *current, line)
}

fn lookup_keyword<'a>(source: &'a str, start: usize, current: usize, line: i32) -> Token {
    let c = source.as_bytes()[start];
    match c {
        b'l' => { return check_keyword(source, start, current, line, "et", 1, 2, Token::Let(line)); }
        b'r' => { return check_keyword(source, start, current, line, "eturn", 1, 5, Token::Return(line)); }
        b'i' => {
            if current - start > 1 {
                match source.as_bytes()[start + 1] {
                    b'f' => { return Token::If(line); }
                    b'n' => { return Token::In(line); }
                    _ => (),
                }
            }
        }
        b't' => { return check_keyword(source, start, current, line, "rue", 1, 3, Token::True(line)); }
        b'p' => { return check_keyword(source, start, current, line, "rint", 1, 4, Token::Print(line)); }
        b'e' => {
            if current - start > 1 {
                match source.as_bytes()[start + 1] {
                    b'n' => { return check_keyword(source, start, current, line, "d", 2, 1, Token::End(line)); }
                    b'l' => { return check_keyword(source, start, current, line, "se", 2, 2, Token::Else(line)); }
                    _ => (),
                }
            }
        }
        b'f' => {
            if current - start > 1 {
                match source.as_bytes()[start + 1] {
                    b'u' => { return check_keyword(source, start, current, line, "n", 2, 1, Token::Fun(line)); }
                    b'a' => { return check_keyword(source, start, current, line, "lse", 2, 3, Token::False(line)); }
                    b'o' => { return check_keyword(source, start, current, line, "r", 2, 1, Token::For(line)); }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    Token::Ident(start, current - start, line)
}

fn check_keyword<'a>(source: &'a str, start: usize, current: usize, line: i32, rest: &str, begin: usize, len: usize, token_type: Token) -> Token {
    if current - start == begin + len {
        let slice = &source[start+begin..start+begin+len];
        if rest == slice {
            return token_type;
        }
    }
    Token::Ident(start, current - start, line)
}

fn num<'a>(source: &'a str, start: usize, current: &mut usize, line: i32) -> Token {
    while is_digit(peek(source, *current)) { *current += 1; }
    if peek(source, *current) == b'.' && is_digit(peek_next(source, *current)) {
        *current += 1;

        while is_digit(peek(source, *current)) { *current += 1; }
    }

    let string = &source[start..*current];
    Token::Num(string.parse().unwrap(), line)
}

fn string<'a>(source: &'a str, start: usize, current: &mut usize, line: &mut i32) -> Token {
    while peek(source, *current) != b'"' && *current < source.len() {
        *current += 1;

        if peek(source, *current) == b'\n' { *line += 1; }
    }

    if *current >= source.len() {
        panic!("unterminated string");
    }

    *current += 1;
    Token::String(start + 1, *current - start - 2, *line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_illegal() {
        let source = "👿";
        let tokens = scan(source);

        let token = tokens.first().unwrap();
        assert_eq!(*token, Token::Illegal(1));
    }

    #[test]
    fn test_token_type() {
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
            Token::Let(1),
            Token::Ident(4, 4, 1),
            Token::Assign(1),
            Token::Num(5.0, 1),
            Token::NewLine(2),
            Token::Let(2),
            Token::Ident(25, 4, 2),
            Token::Assign(2),
            Token::Num(10.5, 2),
            Token::NewLine(3),
            Token::NewLine(4),
            Token::Let(4),
            Token::Ident(50, 6, 4),
            Token::Assign(4),
            Token::String(60, 4, 4),
            Token::Semicolon(4),
            Token::Let(4),
            Token::Ident(71, 13, 4),
            Token::Assign(4),
            Token::String(88, 12, 4),
            Token::NewLine(5),
            Token::NewLine(6),
            Token::Fun(6),
            Token::Ident(115, 3, 6),
            Token::LParen(6),
            Token::Ident(119, 1, 6),
            Token::Comma(6),
            Token::Ident(122, 1, 6),
            Token::RParen(6),
            Token::Colon(6),
            Token::NewLine(7),
            Token::Return(7),
            Token::Ident(145, 1, 7),
            Token::Plus(7),
            Token::Ident(149, 1, 7),
            Token::NewLine(8),
            Token::End(8),
            Token::NewLine(9),
            Token::NewLine(10),
            Token::Let(10),
            Token::Ident(179, 6, 10),
            Token::Assign(10),
            Token::Ident(188, 3, 10),
            Token::LParen(10),
            Token::Ident(192, 4, 10),
            Token::Comma(10),
            Token::Ident(198, 3, 10),
            Token::RParen(10),
            Token::NewLine(11),
            Token::Bang(11),
            Token::Minus(11),
            Token::Slash(11),
            Token::Star(11),
            Token::Lt(11),
            Token::Gt(11),
            Token::LtEq(11),
            Token::GtEq(11),
            Token::Eq(11),
            Token::BangEq(11),
            Token::NewLine(12),
            Token::NewLine(13),
            Token::If(13),
            Token::True(13),
            Token::Colon(13),
            Token::NewLine(14),
            Token::Return(14),
            Token::String(268, 2, 14),
            Token::NewLine(15),
            Token::Else(15),
            Token::If(15),
            Token::False(15),
            Token::Colon(15),
            Token::NewLine(16),
            Token::Return(16),
            Token::String(315, 7, 16),
            Token::NewLine(17),
            Token::Else(17),
            Token::Colon(17),
            Token::NewLine(18),
            Token::Return(18),
            Token::String(358, 4, 18),
            Token::NewLine(19),
            Token::End(19),
            Token::NewLine(20),
            Token::NewLine(21),
            Token::For(21),
            Token::Ident(389, 1, 21),
            Token::In(21),
            Token::Ident(394, 5, 21),
            Token::Colon(21),
            Token::NewLine(22),
            Token::Print(22),
            Token::String(420, 24, 23),
            Token::NewLine(24),
            Token::End(24),
            Token::NewLine(25),
            Token::Eof,
        ];

        let tokens = scan(source);
        for (i, e) in expected.iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e, t, "error position {} expected == {:?} got == {:?}", i, e, t);
        }
    }
}
