use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub(crate) struct Parser<'a> {
    lexer: Lexer<'a>,
    tokens: Vec<Token<'a>>,
    cur_token: Token<'a>,
    read_token: Token<'a>,
    prefix_parse_fns: Vec<Option<PrefixParseFn>>,
}

type PrefixParseFn = for <'a> fn(&'a mut Parser, token: &'a Token) -> Box<dyn Expr<'a>>;

impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            tokens: vec![],
            cur_token: ,
            prefix_parse_fns: vec![
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        }
    }

    fn set_tokens(&mut self) {
        loop {
            let t = self.lexer.next_token();
            self.tokens.push(t);
            if t.token_type == TokenType::Eof { return; }
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.cur_token];
    }

    pub(crate) fn parse_expression(&mut self) -> Box<dyn Expr<'a>> {

        loop {
            let token = self.peek();
            let prefix = self.prefix_parse_fns[token.token_type as usize];
            if prefix == None {
                panic!();
            }

            return prefix.unwrap()(self, &token);
        }
    }

    //Parse Fns

}



#[cfg(test)]
mod tests {
    use crate::ast::{NameExpr, PrefixExpr};
    use super::*;

    #[test]
    fn test_prefix() {
        let source = r#"
        -x
        "#;

        let name_token = Token::new(TokenType::Ident, "x", 2);
        let name_exp = NameExpr::new(name_token);
        let exp = PrefixExpr::new(&TokenType::Minus, Box::new(name_exp));

        let mut l = Lexer::new(source);
        let mut p = Parser::new(l);


        let expr= p.parse_expression();
        let prefix_expr: &PrefixExpr = *expr.as_any().downcast_ref().unwrap();

        let right = &prefix_expr.right;
        let name_expr: &NameExpr = *right.as_any().downcast_ref().unwrap();

        assert_eq!(*prefix_expr.op, TokenType::Minus);
        assert_eq!(name_expr.token.lexeme, "x");
        assert_eq!(name_expr.token.token_type, TokenType::Ident);
        assert_eq!(name_expr.token.line, 2);
    }
}