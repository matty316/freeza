use std::any::Any;
use crate::token::*;

pub(crate) trait Expr<'a> {
    fn as_any(&self) -> &(dyn Any + 'a);
}

pub(crate) struct NameExpr<'a> {
    pub(crate) token: Token<'a>,
}

impl<'a> NameExpr<'a> {
    pub(crate) fn new(token: Token::<'a>) -> Self {
        NameExpr {
            token
        }
    }
}

impl<'a> Expr<'a> for NameExpr<'a> {
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}

pub(crate) struct PrefixExpr<'a> {
    pub(crate) op: &'a TokenType,
    pub(crate) right: Box<dyn Expr<'a>>,
}

impl<'a> PrefixExpr<'a> {
    pub(crate) fn new(op: &'a TokenType, right: Box<dyn Expr<'a>>) -> Self {
        PrefixExpr {
            op, right,
        }
    }
}

impl<'a> Expr<'a> for PrefixExpr<'a> {
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}