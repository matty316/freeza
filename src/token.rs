
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    Ident,
    Num,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Eq,
    BangEq,
    Bang,
    Assign,
    Let,
    Fun,
    If,
    Else,
    LParen,
    RParen,
    Colon,
    Semicolon,
    String,
    NewLine,
    Return,
    End,
    For,
    In,
    True,
    False,
    Print,
    Illegal,
    Eof
}

#[derive(Debug, Clone)]
pub(crate) struct Token<'a> {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: &'a str,
    pub(crate) line: i32,
}

impl<'a> Token<'a> {
    pub(crate) fn new(token_type: TokenType, lexeme: &'a str, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}