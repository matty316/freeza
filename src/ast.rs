use crate::token::*;

pub(crate) trait Node {
    fn string(&self) -> String;
}

pub(crate) trait Stmt: Node {
}

pub(crate) trait Expr: Node {
}

pub(crate)struct Program {
    pub(crate) stmts: Vec<Box<dyn Stmt>>,
}

impl Program {
    pub(crate) fn new() -> Program {
        Program {
            stmts: vec![]
        }
    }

    pub(crate) fn string(&self) -> String {
        let mut string = "".to_string();
        for stmt in &self.stmts {
            string += &stmt.string()
        }
        return string
    }
}

pub(crate) struct LetStmt {
    pub(crate) token: Token,
    pub(crate) name: Identifier,
    pub(crate) value: dyn Expr,
}

impl Node for LetStmt {
    fn string(&self) -> String { format!("let {} = {}", self.name.string(), self.value.string())}
}

pub(crate) struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn string(&self) -> String { self.value.clone() }
}