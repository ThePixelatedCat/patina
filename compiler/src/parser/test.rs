use super::Parser;
use super::ast::{Ast, Binding, Bop, Expr, Field, Item, Lit, Type, Unop, Variant};

fn parse_expr(input: &str) -> Expr {
    let mut parser = Parser::new(input);
    parser.expression().unwrap()
}

// fn parse_stmt(input: &str) -> Stmt {
//     let mut parser = Parser::new(input);
//     parser.statement().unwrap()
// }

fn parse_item(input: &str) -> Item {
    let mut parser = Parser::new(input);
    parser.item().unwrap()
}

fn parse_ast(input: &str) -> Ast {
    let mut parser = Parser::new(input);
    parser.file().unwrap()
}

#[test]
fn parse_expression() {
    let expr = parse_expr("42");
    assert_eq!(expr, Lit::Int(42).into());

    let expr = parse_expr("  2.7768");
    assert_eq!(expr, Lit::Float(2.7768).into());

    let expr = parse_expr(r#""I am a Str!""#);
    assert_eq!(expr, Lit::Str("I am a Str!".into()).into());

    let expr = parse_expr("foo");
    assert_eq!(expr, Expr::Ident("foo".into()));

    let expr = parse_expr("bar (  x, 2)");
    assert_eq!(
        expr,
        Expr::FnCall {
            fun: Expr::Ident("bar".into()).into(),
            args: vec![Expr::Ident("x".into()), Lit::Int(2).into(),],
        }
    );

    let expr = parse_expr("!  is_visible");
    assert_eq!(
        expr,
        Expr::UnaryOp {
            op: Unop::Not,
            expr: Expr::Ident("is_visible".into()).into(),
        }
    );

    let expr = parse_expr("-(-13)");
    assert_eq!(
        expr,
        Expr::UnaryOp {
            op: Unop::Neg,
            expr: Lit::Int(-13).into(),
        }
    );

    let expr = parse_expr("if (0.5) foo()");
    assert_eq!(
        expr,
        Expr::If {
            cond: Lit::Float(0.5).into(),
            th: Expr::FnCall {
                fun: Expr::Ident("foo".into()).into(),
                args: Vec::new()
            }
            .into(),
            el: None
        }
    );

    let expr = parse_expr("if (0.5) foo else bar");
    assert_eq!(
        expr,
        Expr::If {
            cond: Lit::Float(0.5).into(),
            th: Expr::Ident("foo".into()).into(),
            el: Some(Expr::Ident("bar".into()).into())
        }
    );
}

#[test]
fn parse_binary_expressions() {
    let expr = parse_expr("4 + 2 * 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Add,
            lhs: Lit::Int(4).into(),
            rhs: Box::new(Expr::BinaryOp {
                op: Bop::Mul,
                lhs: Lit::Int(2).into(),
                rhs: Lit::Int(3).into()
            })
        }
    );

    let expr = parse_expr("4 * 2 + 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Add,
            lhs: Box::new(Expr::BinaryOp {
                op: Bop::Mul,
                lhs: Lit::Int(4).into(),
                rhs: Lit::Int(2).into()
            }),
            rhs: Lit::Int(3).into(),
        }
    );

    let expr = parse_expr("4 - 2 - 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Sub,
            lhs: Box::new(Expr::BinaryOp {
                op: Bop::Sub,
                lhs: Lit::Int(4).into(),
                rhs: Lit::Int(2).into()
            }),
            rhs: Lit::Int(3).into(),
        }
    );

    let expr = parse_expr("4 ** 2 ** 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Exp,
            lhs: Lit::Int(4).into(),
            rhs: Box::new(Expr::BinaryOp {
                op: Bop::Exp,
                lhs: Lit::Int(2).into(),
                rhs: Lit::Int(3).into()
            })
        }
    );

    let expr = parse_expr("4 ^ 2 ^ 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Xor,
            lhs: Box::new(Expr::BinaryOp {
                op: Bop::Xor,
                lhs: Lit::Int(4).into(),
                rhs: Lit::Int(2).into()
            }),
            rhs: Lit::Int(3).into(),
        }
    );
}

#[test]
fn parse_statements() {
    let stmt = parse_expr("let x = 7 + sin(3.);");
    assert_eq!(
        stmt,
        Expr::Let {
            binding: Binding {
                mutable: false,
                name: "x".into(),
                type_annotation: None
            },
            value: Expr::BinaryOp {
                op: Bop::Add,
                lhs: Lit::Int(7).into(),
                rhs: Expr::FnCall {
                    fun: Expr::Ident("sin".into()).into(),
                    args: vec![Lit::Float(3.0).into()]
                }
                .into()
            }.into()
        }
    );

    let stmt = parse_expr("let mut y: Int = 7");
    assert_eq!(
        stmt,
        Expr::Let {
            binding: Binding {
                mutable: true,
                name: "y".into(),
                type_annotation: Some(Type {
                    name: "Int".into(),
                    generics: vec![]
                })
            },
            value: Lit::Int(7).into()
        }
    );

    let stmt = parse_expr("y = 3 + 7 * 0.5");
    assert_eq!(
        stmt,
        Expr::Assign {
            ident: "y".into(),
            value: Expr::BinaryOp {
                op: Bop::Add,
                lhs: Lit::Int(3).into(),
                rhs: Expr::BinaryOp {
                    op: Bop::Mul,
                    lhs: Lit::Int(7).into(),
                    rhs: Lit::Float(0.5).into()
                }
                .into()
            }.into()
        }
    );
}

#[test]
fn parse_const() {
    let item = parse_item(r#"const HELLO_WORLD: Str = "Hello, World!""#);
    assert_eq!(
        item,
        Item::Const {
            ident: "HELLO_WORLD".into(),
            ty: Type {
                name: "Str".into(),
                generics: vec![]
            },
            value: Lit::Str("Hello, World!".into()).into()
        }
    );
}

#[test]
fn parse_struct() {
    let item = parse_item(
        r#"
        struct Foo<T, U> {
            x: Str,
            bar: Bar<Baz<T>>
        }
    "#,
    );
    assert_eq!(
        item,
        Item::Struct {
            name: Type {
                name: "Foo".into(),
                generics: vec![
                    Type {
                        name: "T".into(),
                        generics: vec![]
                    },
                    Type {
                        name: "U".into(),
                        generics: vec![]
                    }
                ]
            },
            fields: vec![
                Field {
                    name: "x".into(),
                    ty: Type {
                        name: "Str".into(),
                        generics: vec![]
                    }
                },
                Field {
                    name: "bar".into(),
                    ty: Type {
                        name: "Bar".into(),
                        generics: vec![Type {
                            name: "Baz".into(),
                            generics: vec![Type {
                                name: "T".into(),
                                generics: vec![]
                            }]
                        }]
                    }
                }
            ]
        }
    )
}

#[test]
fn parse_enum() {
    let item = parse_item(
        r#"
        enum Foo {
            X,
            Y(Bar),
            Z { baz:Baz, fizz: Buzz }
        }
    "#,
    );
    assert_eq!(
        item,
        Item::Enum {
            name: Type {
                name: "Foo".into(),
                generics: vec![]
            },
            variants: vec![
                Variant::Unit("X".into()),
                Variant::Tuple(
                    "Y".into(),
                    vec![Type {
                        name: "Bar".into(),
                        generics: vec![]
                    }]
                ),
                Variant::Struct(
                    "Z".into(),
                    vec![
                        Field {
                            name: "baz".into(),
                            ty: Type {
                                name: "Baz".into(),
                                generics: vec![]
                            }
                        },
                        Field {
                            name: "fizz".into(),
                            ty: Type {
                                name: "Buzz".into(),
                                generics: vec![]
                            }
                        }
                    ]
                ),
            ]
        }
    )
}

#[test]
fn parse_function() {
    let item = parse_item(
        r#"
        fn foo(mut a, b: Int) -> a + b
    "#,
    );
    assert_eq!(
        item,
        Item::Function {
            name: "foo".into(),
            params: vec![
                Binding {
                    mutable: true,
                    name: "a".into(),
                    type_annotation: None
                },
                Binding {
                    mutable: false,
                    name: "b".into(),
                    type_annotation: Some(Type {
                        name: "Int".into(),
                        generics: vec![]
                    })
                }
            ],
            return_type: None,
            body: Expr::BinaryOp {
                op: Bop::Add,
                lhs: Expr::Ident("a".into()).into(),
                rhs: Expr::Ident("b".into()).into()
            }
        }
    )
}

// fn wow_we_did_it(x: String, bar: Bar<Baz<T>, U>): Foo -> {
//     let mut x = 7 + sin(y);
//     x = if (bar < 3)
//         x + 1
//     else if (bar < 2)
//         2 ** 3
//     else
//         1;
// }
#[test]
fn parse_file() {
    let items = parse_ast(
        r#"
        fn wow_we_did_it(mut x, bar: Bar<Baz<T>, U>): Foo -> 
            if (bar < 3) 
                x + 1
            else if (bar <= 2) 
                fizz(3, 5.1)

        struct Foo<T, U> {
            x: Str,
            bar: Bar<Baz<T>, U>,
        }
    "#,
    );

    assert_eq!(
        items[0],
        Item::Function {
            name: "wow_we_did_it".into(),
            params: vec![
                Binding {
                    mutable: true,
                    name: "x".into(),
                    type_annotation: None
                },
                Binding {
                    mutable: false,
                    name: "bar".into(),
                    type_annotation: Some(Type {
                        name: "Bar".into(),
                        generics: vec![
                            Type {
                                name: "Baz".into(),
                                generics: vec![Type {
                                    name: "T".into(),
                                    generics: vec![],
                                }],
                            },
                            Type {
                                name: "U".into(),
                                generics: vec![],
                            }
                        ],
                    })
                }
            ],
            return_type: Some(Type {
                name: "Foo".into(),
                generics: vec![]
            }),
            body: Expr::If {
                cond: Expr::BinaryOp {
                    op: Bop::Lt,
                    lhs: Expr::Ident("bar".into()).into(),
                    rhs: Lit::Int(3).into()
                }
                .into(),
                th: Expr::BinaryOp {
                    op: Bop::Add,
                    lhs: Expr::Ident("x".into()).into(),
                    rhs: Lit::Int(1).into()
                }
                .into(),
                el: Some(
                    Expr::If {
                        cond: Expr::BinaryOp {
                            op: Bop::Leq,
                            lhs: Expr::Ident("bar".into()).into(),
                            rhs: Lit::Int(2).into()
                        }
                        .into(),
                        th: Expr::FnCall {
                            fun: Expr::Ident("fizz".into()).into(),
                            args: vec![Lit::Int(3).into(), Lit::Float(5.1).into()]
                        }
                        .into(),
                        el: None
                    }
                    .into()
                )
            }
        }
    );

    assert_eq!(
        items[1],
        Item::Struct {
            name: Type {
                name: "Foo".into(),
                generics: vec![
                    Type {
                        name: "T".into(),
                        generics: vec![],
                    },
                    Type {
                        name: "U".into(),
                        generics: vec![],
                    }
                ],
            },
            fields: vec![
                Field {
                    name: "x".into(),
                    ty: Type {
                        name: "Str".into(),
                        generics: vec![],
                    },
                },
                Field {
                    name: "bar".into(),
                    ty: Type {
                        name: "Bar".into(),
                        generics: vec![
                            Type {
                                name: "Baz".into(),
                                generics: vec![Type {
                                    name: "T".into(),
                                    generics: vec![],
                                }],
                            },
                            Type {
                                name: "U".into(),
                                generics: vec![],
                            }
                        ],
                    },
                }
            ]
        }
    );
}
