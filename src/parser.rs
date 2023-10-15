use crate::token::*;
use crate::ast::*;
use crate::lexer::*;

struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub(crate)fn new(&self, l: &'a mut Lexer) -> Self {
        let p = Parser {
            lexer: l,
            current_token: Token { token_type: TokenType::Eof, literal: "".to_string(), line: 1 },
            peek_token: Token { token_type: TokenType::Eof, literal: "".to_string(), line: 1 },
            errors: vec![],
        };

        return p;
    }

    pub(crate)fn parse(&self) -> Program {
        let program = Program::new();

        return program;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
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
        let num = 1
        let num2 = 10.2
        let condition = true
        "###;

        let l = Lexer::new(input.to_string());
        let p = Parser::new(l);
        let program = p.parse();

        let exp: Vec<(&str, Box<dyn Any>)> = vec![
            ("string", "string"),
            ("num", 1),
            ("num2", 10.2),
            ("condition", true),
        ];
    }
}