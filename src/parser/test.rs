use super::Parser;
use super::ast::{Bop, Expr, Lit, Unop};

fn parse(input: &str) -> Expr {
    let mut parser = Parser::new(input);
    parser.expression()
}

#[test]
fn parse_expression() {
    let expr = parse("42");
    assert_eq!(expr, Expr::Literal(Lit::Int(42)));

    let expr = parse("  2.7768");
    assert_eq!(expr, Expr::Literal(Lit::Float(2.7768)));

    let expr = parse(r#""I am a String!""#);
    assert_eq!(expr, Expr::Literal(Lit::Str("I am a String!".into())));

    let expr = parse("foo");
    assert_eq!(expr, Expr::Ident("foo".to_string()));

    let expr = parse("!  is_visible");
    assert_eq!(
        expr,
        Expr::UnaryOp {
            op: Unop::Not,
            expr: Box::new(Expr::Ident("is_visible".to_string())),
        }
    );

    let expr = parse("-(-13)");
    assert_eq!(
        expr,
        Expr::UnaryOp {
            op: Unop::Neg,
            expr: Box::new(Expr::Literal(Lit::Int(-13))),
        }
    );
}

#[test]
fn parse_binary_expressions() {
    let expr = parse("4 + 2 * 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Add,
            lhs: Box::new(Expr::Literal(Lit::Int(4))),
            rhs: Box::new(Expr::BinaryOp {
                op: Bop::Mul,
                lhs: Box::new(Expr::Literal(Lit::Int(2))),
                rhs: Box::new(Expr::Literal(Lit::Int(3)))
            })
        }
    );

    let expr = parse("4 * 2 + 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Add,
            lhs: Box::new(Expr::BinaryOp {
                op: Bop::Mul,
                lhs: Box::new(Expr::Literal(Lit::Int(4))),
                rhs: Box::new(Expr::Literal(Lit::Int(2)))
            }),
            rhs: Box::new(Expr::Literal(Lit::Int(3))),
        }
    );

    let expr = parse("4 - 2 - 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Sub,
            lhs: Box::new(Expr::BinaryOp {
                op: Bop::Sub,
                lhs: Box::new(Expr::Literal(Lit::Int(4))),
                rhs: Box::new(Expr::Literal(Lit::Int(2)))
            }),
            rhs: Box::new(Expr::Literal(Lit::Int(3))),
        }
    );

    let expr = parse("4 ^ 2 ^ 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Exp,
            lhs: Box::new(Expr::Literal(Lit::Int(4))),
            rhs: Box::new(Expr::BinaryOp {
                op: Bop::Exp,
                lhs: Box::new(Expr::Literal(Lit::Int(2))),
                rhs: Box::new(Expr::Literal(Lit::Int(3)))
            })
        }
    );
}
