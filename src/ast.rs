use crate::token::*;

trait Node {
    fn string(&self) -> String;
}

trait Stmt: Node {
}

trait Expr: Node {
}

pub(crate)struct Program {
    stmts: Vec<Box<dyn Stmt>>,
}

impl Program {
    pub(crate)fn new() -> Program {
        Program {
            stmts: vec![]
        }
    }

    fn string(&self) -> String {
        let mut string = "".to_string();
        for stmt in &self.stmts {
            string += &stmt.string()
        }
        return string
    }
}

struct LetStmt {
    token: Token,
    name: Identifier,
    value: dyn Expr,
}

impl Node for LetStmt {
    fn string(&self) -> String { format!("let {} = {}", self.name.string(), self.value.string())}
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn string(&self) -> String { self.value.clone() }
}