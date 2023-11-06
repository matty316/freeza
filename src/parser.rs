use crate::ast::*;
use crate::lexer::*;
use crate::token::{Token};

fn parse<'a>(tokens: Vec<Token<'a>>) -> Box<dyn Expr<'a>> {
    for t in tokens {
        match t {
            Token::Minus => { return parse_prefix(&t, tokens); }
            _ => (),
        }
    }

    return parse(tokens);
}

fn parse_prefix<'a>(token: &'a Token<'a>, tokens: Vec<Token<'a>>) -> Box<dyn Expr<'a>> {
    let right = parse(tokens);
    let prefix = PrefixExpr::new(&token, right);
    Box::new(prefix)
}

#[cfg(test)]
mod tests {
    use crate::ast::{NameExpr, PrefixExpr};
    use crate::lexer::scan;
    use super::*;

    #[test]
    fn test_prefix() {
        let source = r#"
        -x
        "#;


    }
}