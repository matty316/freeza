
#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Ident(usize, usize, i32),
    Num(f64, i32),
    Comma(i32),
    Plus(i32),
    Minus(i32),
    Star(i32),
    Slash(i32),
    Lt(i32),
    Gt(i32),
    LtEq(i32),
    GtEq(i32),
    Eq(i32),
    BangEq(i32),
    Bang(i32),
    Assign(i32),
    Let(i32),
    Fun(i32),
    If(i32),
    Else(i32),
    LParen(i32),
    RParen(i32),
    Colon(i32),
    Semicolon(i32),
    String(usize, usize, i32),
    NewLine(i32),
    Return(i32),
    End(i32),
    For(i32),
    In(i32),
    True(i32),
    False(i32),
    Print(i32),
    Illegal(i32),
    Eof
}