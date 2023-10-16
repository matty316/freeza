use crate::token::*;
use crate::ast::*;
use crate::lexer::*;

struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    current_token: usize,
    peek_token: usize,
}

impl Parser {
    pub(crate)fn new(l: Lexer) -> Self {
        let p = Parser {
            lexer: l,
            errors: vec![],
            current_token: 0,
            peek_token: 1,
        };

        let mut ct = l.next_token(ch, position, read_position, line);
        p.next_token(current_token, peek_token, ch, position, read_position, line);

        return p;
    }

    pub(crate)fn parse(&self) -> Program {
        let program = Program::new();

        return program;
    }

    fn next_token(&self, current_token: &mut Token, peek_token: &mut Token, ch: &mut u8, position: &mut usize, read_position: &mut usize, line: &mut u32) {
        *current_token = peek_token.clone();
        *peek_token = self.lexer.next_token(ch, position, read_position, line)
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
            ("string", Box::new("string")),
            ("num", Box::new(1)),
            ("num2", Box::new(10.2)),
            ("condition", Box::new(true)),
        ];

        for (i, e) in exp.iter().enumerate() {
            let stmt = &program.stmts[i];
            assert_eq!(e.0, &stmt.string())
        }
    }
}