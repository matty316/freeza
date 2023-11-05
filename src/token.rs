
#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub(crate) struct Token<'a> {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: &'a str,
    pub(crate) line: i32,
}