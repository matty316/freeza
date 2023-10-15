use std::clone;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    Ident,
    Int,
    Float,
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
    Illegal,
    Eof
}

#[derive(Clone)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
    pub(crate) line: u32,
}