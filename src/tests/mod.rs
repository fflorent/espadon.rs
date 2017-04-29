use super::*;
use nom::{IResult};

mod literals;
mod expressions;
mod statements;

#[test]
fn it_parses_multiple_statements() {
    assert_eq!(parse("var test1; 42;"), IResult::Done("", Program {
        body: vec![
            Statement::VariableDeclaration(VariableDeclaration {
                declarations: vec![VariableDeclarator {
                    id: "test1".to_string(),
                    init: None
                }],
                kind: "var".to_string()
            }),
            Statement::Expression(Expression::Literal(Literal {
                value: LiteralValue::Number(42.0)
            }))
        ]
    }));
}

#[test]
fn it_parses_statements_with_whitespaces_around() {
    assert_eq!(parse(" var test;"), parse("var test;"));
    assert_eq!(parse(" var   test   ;"), parse("var test;"));
    assert_eq!(parse(" var   test   ;   ").to_result(), parse("var test;").to_result());
}
