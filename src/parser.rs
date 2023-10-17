use crate::token::*;
use crate::ast::*;
use crate::lexer::*;

struct Parser<'a> {
    lexer: &'a mut Lexer,
    errors: Vec<String>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub(crate)fn new(l: &'a mut Lexer) -> Self {
        let mut p: Parser<'a> = Parser {
            lexer: l,
            errors: vec![],
            current_token: Token { token_type: TokenType::Illegal, literal: "".to_string(), line: 1 },
            peek_token: Token { token_type: TokenType::Illegal, literal: "".to_string(), line: 1 },
        };

        p.next_token();
        p.next_token();

        return p;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    pub(crate)fn parse(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::Eof {
            let stmt = self.parse_statement();

            match stmt {
                Ok(s) => program.stmts.push(s),
                Err(e) => eprint!("{}", e),
            }

            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Stmt>, &str> {
        match self.current_token.token_type {
            TokenType::Let => Ok(self.parse_let()?),
            _ => Err("error!"),
        }
    }

    fn parse_let(&mut self) -> Result<Box<LetStmt>, &str> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            return Err("error!");
        }

        let name = Identifier { token: self.current_token.clone(), value: self.current_token.clone().literal };
        
        if !self.expect_peek(TokenType::Assign) {
            return Err("error!");
        }

        while !self.current_token_is(TokenType::NewLine) && !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = LetStmt {token: token, name: name};
        Ok(Box::new(stmt))
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[test]
    fn testLet() {
        let input = r###"
        let name = "string"
        let num = 1; let num2 = 10.2
        let condition = true
        "###;

        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse();

        let exp: Vec<(&str, Box<dyn Any>)> = vec![
            ("string", Box::new("string")),
            ("num", Box::new(1)),
            ("num2", Box::new(10.2)),
            ("condition", Box::new(true)),
        ];

        assert_eq!(program.stmts.len(), 4);
        // for (i, e) in exp.iter().enumerate() {
        //     let stmt = &program.stmts[i];
        //     assert_eq!(e.0, &stmt.string())
        // }
    }
}