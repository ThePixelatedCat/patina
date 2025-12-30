use crate::span::Spannable;

use super::Parser;
use super::ast::{ExprS, ItemS, Ast, Binding, Bop, Expr, Field, Item, Type, Unop, Variant};

fn parse_expr(input: &str) -> ExprS {
    let mut parser = Parser::new(input);
    parser.expression().unwrap()
}

fn parse_item(input: &str) -> ItemS {
    let mut parser = Parser::new(input);
    parser.item().unwrap()
}

fn parse_ast(input: &str) -> Ast {
    let mut parser = Parser::new(input);
    parser.file().unwrap()
}

#[test]
fn parse_lit_expressions() {
    let expr = parse_expr("42");
    assert_eq!(expr, Expr::Int(42).spanned(0..2));

    let expr = parse_expr("  2.7768");
    assert_eq!(expr, Expr::Float(2.7768).spanned(2..8));

    let expr = parse_expr(r#""I am a Str!""#);
    assert_eq!(expr, Expr::Str("I am a Str!".into()).spanned(0..13));

    let expr = parse_expr(r#"(42,(2,),"end")"#);
    assert_eq!(
        expr,
        Expr::Tuple(vec![
            Expr::Int(42).spanned(1..3),
            Expr::Tuple(vec![Expr::Int(2).spanned(5..6)]).spanned(4..8),
            Expr::Str("end".into()).spanned(9..14)
        ]).spanned(0..15)
    );

    let expr = parse_expr("[1, 4, 3, 2]");
    assert_eq!(
        expr,
        Expr::Array(vec![
            Expr::Int(1).spanned(1..2),
            Expr::Int(4).spanned(4..5),
            Expr::Int(3).spanned(7..8),
            Expr::Int(2).spanned(10..11)
        ]).spanned(0..12)
    );

    let expr = parse_expr("foo");
    assert_eq!(expr, Expr::Ident("foo".into()).spanned(0..3));
}

#[test]
fn parse_unop_expressions() {
    let expr = parse_expr("!  is_visible");
    assert_eq!(
        expr,
        Expr::UnaryOp {
            op: Unop::Not,
            expr: Expr::Ident("is_visible".into()).spanned(3..13).into(),
        }.spanned(0..13)
    );

    let expr = parse_expr("-(-13)");
    assert_eq!(
        expr,
        Expr::UnaryOp {
            op: Unop::Neg,
            expr: Expr::UnaryOp {
                op: Unop::Neg,
                expr: Expr::Int(13).spanned(3..5).into(),
            }
            .spanned(1..6)
            .into()
        }.spanned(0..6)
    );
}

#[test]
fn parse_binop_expressions() {
    let expr = parse_expr("4 + 2 * 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Add,
            lhs: Expr::Int(4).spanned(0..1).into(),
            rhs: Expr::BinaryOp {
                op: Bop::Mul,
                lhs: Expr::Int(2).spanned(4..5).into(),
                rhs: Expr::Int(3).spanned(8..9).into()
            }.spanned(4..9).into()
        }.spanned(0..9)
    );

    let expr = parse_expr("4 * 2 + 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Add,
            lhs: Expr::BinaryOp {
                op: Bop::Mul,
                lhs: Expr::Int(4).spanned(0..1).into(),
                rhs: Expr::Int(2).spanned(4..5).into()
            }.spanned(0..5).into(),
            rhs: Expr::Int(3).spanned(8..9).into(),
        }.spanned(0..9)
    );

    let expr = parse_expr("4 - 2 - 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Sub,
            lhs: Expr::BinaryOp {
                op: Bop::Sub,
                lhs: Expr::Int(4).spanned(0..1).into(),
                rhs: Expr::Int(2).spanned(4..5).into()
            }.spanned(0..5).into(),
            rhs: Expr::Int(3).spanned(8..9).into(),
        }.spanned(0..9)
    );

    let expr = parse_expr("4 ** 2 ** 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Exp,
            lhs: Expr::Int(4).spanned(0..1).into(),
            rhs: Expr::BinaryOp {
                op: Bop::Exp,
                lhs: Expr::Int(2).spanned(5..6).into(),
                rhs: Expr::Int(3).spanned(10..11).into()
            }.spanned(5..11).into()
        }.spanned(0..11)
    );

    let expr = parse_expr("4 ^ 2 ^ 3");
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: Bop::Xor,
            lhs: Expr::BinaryOp {
                op: Bop::Xor,
                lhs: Expr::Int(4).spanned(0..1).into(),
                rhs: Expr::Int(2).spanned(4..5).into()
            }.spanned(0..5).into(),
            rhs: Expr::Int(3).spanned(8..9).into(),
        }.spanned(0..9)
    );
}

#[test]
fn parse_compound_expressions() {
    let expr = parse_expr("bar (  x, 2)");
    assert_eq!(
        expr,
        Expr::FnCall {
            fun: Expr::Ident("bar".into()).spanned(0..3).into(),
            args: vec![Expr::Ident("x".into()).spanned(7..8), Expr::Int(2).spanned(10..11),],
        }.spanned(0..12)
    );

    let expr = parse_expr("if (0.5) foo()");
    assert_eq!(
        expr,
        Expr::If {
            cond: Expr::Float(0.5).spanned(4..7).into(),
            th: Expr::FnCall {
                fun: Expr::Ident("foo".into()).spanned(9..12).into(),
                args: Vec::new()
            }.spanned(9..14)
            .into(),
            el: None
        }.spanned(0..14)
    );

    let expr = parse_expr("if (0.5) foo else bar");
    assert_eq!(
        expr,
        Expr::If {
            cond: Expr::Float(0.5).spanned(4..7).into(),
            th: Expr::Ident("foo".into()).spanned(9..12).into(),
            el: Some(Expr::Ident("bar".into()).spanned(18..21).into())
        }.spanned(0..21)
    );

    let expr = parse_expr("(|a, b: Int| -> a + b)(1, 2)");
    assert_eq!(
        expr,
        Expr::FnCall {
            fun: Expr::Lambda {
                params: vec![
                    Binding {
                        mutable: false,
                        name: "a".into(),
                        type_annotation: None
                    }.spanned(2..3),
                    Binding {
                        mutable: false,
                        name: "b".into(),
                        type_annotation: Some(Type::Ident {
                            name: "Int".into(),
                            generics: vec![]
                        }.spanned(8..11))
                    }.spanned(5..11)
                ],
                return_type: None,
                body: Expr::BinaryOp {
                    op: Bop::Add,
                    lhs: Expr::Ident("a".into()).spanned(16..17).into(),
                    rhs: Expr::Ident("b".into()).spanned(20..21).into()
                }
                .spanned(16..21)
                .into()
            }
            .spanned(0..22)
            .into(),
            args: vec![Expr::Int(1).spanned(23..24).into(), Expr::Int(2).spanned(26..27).into()]
        }.spanned(0..28)
    );

    let expr = parse_expr("[1, 2, 3][1-1]");
    assert_eq!(
        expr,
        Expr::Index {
            arr: Expr::Array(vec![
                Expr::Int(1).spanned(1..2),
                Expr::Int(2).spanned(4..5),
                Expr::Int(3).spanned(7..8)
            ])
            .spanned(0..9)
            .into(),
            index: Expr::BinaryOp {
                op: Bop::Sub,
                lhs: Expr::Int(1).spanned(10..11).into(),
                rhs: Expr::Int(1).spanned(12..13).into()
            }
            .spanned(10..13)
            .into()
        }
        .spanned(0..14)
    );

    let expr = parse_expr("self._0");
    assert_eq!(
        expr,
        Expr::FieldAccess {
            base: Expr::Ident("self".into()).spanned(0..4).into(),
            field: "_0".into()
        }.spanned(0..7)
    );
}

#[test]
fn parse_var_expresssions() {
    let expr = parse_expr("let x = 7 + sin(3.);");
    assert_eq!(
        expr,
        Expr::Let {
            binding: Binding {
                mutable: false,
                name: "x".into(),
                type_annotation: None
            }.spanned(4..5),
            value: Expr::BinaryOp {
                op: Bop::Add,
                lhs: Expr::Int(7).spanned(8..9).into(),
                rhs: Expr::FnCall {
                    fun: Expr::Ident("sin".into()).spanned(12..15).into(),
                    args: vec![Expr::Float(3.0).spanned(16..18)]
                }
                .spanned(12..19)
                .into()
            }
            .spanned(8..19)
            .into()
        }.spanned(0..19)
    );

    let expr = parse_expr("let mut y: Int = 7");
    assert_eq!(
        expr,
        Expr::Let {
            binding: Binding {
                mutable: true,
                name: "y".into(),
                type_annotation: Some(Type::Ident {
                    name: "Int".into(),
                    generics: vec![]
                }.spanned(11..14))
            }.spanned(4..14),
            value: Expr::Int(7).spanned(17..18).into()
        }.spanned(0..18)
    );

    let expr = parse_expr("y = 3 + 7 * 0.5");
    assert_eq!(
        expr,
        Expr::Assign {
            name: "y".into(),
            value: Expr::BinaryOp {
                op: Bop::Add,
                lhs: Expr::Int(3).spanned(4..5).into(),
                rhs: Expr::BinaryOp {
                    op: Bop::Mul,
                    lhs: Expr::Int(7).spanned(8..9).into(),
                    rhs: Expr::Float(0.5).spanned(12..15).into()
                }
                .spanned(8..15)  
                .into()
            }
            .spanned(4..15)
            .into()
        }
        .spanned(0..15)
    );
}

// #[test]
// fn parse_block_expressions() {
//     let expr = parse_expr(
//         "{
//         let mut y = 5;
//         3 + 1 - 2;
//         y = 1;
//         if (y < 3) {
//             let a = 5;
//             a
//         } else 32;
//     }",
//     );
//     assert_eq!(
//         expr,
//         Expr::Block {
//             exprs: vec![
//                 Expr::Let {
//                     binding: Binding {
//                         mutable: true,
//                         name: "y".into(),
//                         type_annotation: None
//                     },
//                     value: Expr::Int(5)
//                 },
//                 Expr::BinaryOp {
//                     op: Bop::Sub,
//                     lhs: Expr::BinaryOp {
//                         op: Bop::Add,
//                         lhs: Expr::Int(3),
//                         rhs: Expr::Int(1)
//                     }
//                     .into(),
//                     rhs: Expr::Int(2)
//                 },
//                 Expr::BinaryOp {
//                     op: Bop::Assign,
//                     lhs: Expr::Ident("y".into()).into(),
//                     rhs: Expr::Int(1)
//                 },
//                 Expr::If {
//                     cond: Expr::BinaryOp {
//                         op: Bop::Lt,
//                         lhs: Expr::Ident("y".into()).into(),
//                         rhs: Expr::Int(3)
//                     }
//                     .into(),
//                     th: Expr::Block {
//                         exprs: vec![
//                             Expr::Let {
//                                 binding: Binding {
//                                     mutable: false,
//                                     name: "a".into(),
//                                     type_annotation: None
//                                 },
//                                 value: Expr::Int(5)
//                             },
//                             Expr::Ident("a".to_string())
//                         ],
//                         trailing: true
//                     }
//                     .into(),
//                     el: Some(Expr::Int(32))
//                 }
//             ],
//             trailing: false
//         }
//     );
// }

// #[test]
// fn parse_const_items() {
//     let item = parse_item(r#"const HELLO_WORLD: Str = "Hello, World!""#);
//     assert_eq!(
//         item,
//         Item::Const {
//             ident: "HELLO_WORLD".into(),
//             ty: Type::Ident {
//                 name: "Str".into(),
//                 generics: vec![]
//             },
//             value: Expr::Str("Hello, World!".into())
//         }
//     );
// }

// #[test]
// fn parse_struct_items() {
//     let item = parse_item(
//         r#"
//         struct Foo<T, U> {
//             x: Str,
//             bar: Bar<Baz<T>>
//         }
//     "#,
//     );
//     assert_eq!(
//         item,
//         Item::Struct {
//             name: "Foo".into(),
//             generic_params: vec!["T".into(), "U".into()],
//             fields: vec![
//                 Field {
//                     name: "x".into(),
//                     ty: Type::Ident {
//                         name: "Str".into(),
//                         generics: vec![]
//                     }
//                 },
//                 Field {
//                     name: "bar".into(),
//                     ty: Type::Ident {
//                         name: "Bar".into(),
//                         generics: vec![Type::Ident {
//                             name: "Baz".into(),
//                             generics: vec![Type::Ident {
//                                 name: "T".into(),
//                                 generics: vec![]
//                             }]
//                         }]
//                     }
//                 }
//             ]
//         }
//     )
// }

// #[test]
// fn parse_enum_items() {
//     let item = parse_item(
//         r#"
//         enum Foo {
//             X,
//             Y(Bar),
//             Z { baz:Baz, fizz: Buzz }
//         }
//     "#,
//     );
//     assert_eq!(
//         item,
//         Item::Enum {
//             name: "Foo".into(),
//             generic_params: vec![],
//             variants: vec![
//                 Variant::Unit("X".into()),
//                 Variant::Tuple(
//                     "Y".into(),
//                     vec![Type::Ident {
//                         name: "Bar".into(),
//                         generics: vec![]
//                     }]
//                 ),
//                 Variant::Struct(
//                     "Z".into(),
//                     vec![
//                         Field {
//                             name: "baz".into(),
//                             ty: Type::Ident {
//                                 name: "Baz".into(),
//                                 generics: vec![]
//                             }
//                         },
//                         Field {
//                             name: "fizz".into(),
//                             ty: Type::Ident {
//                                 name: "Buzz".into(),
//                                 generics: vec![]
//                             }
//                         }
//                     ]
//                 ),
//             ]
//         }
//     )
// }

// #[test]
// fn parse_function_items() {
//     let item = parse_item(
//         r#"
//         fn foo(mut a, b: Int) -> a + b
//     "#,
//     );
//     assert_eq!(
//         item,
//         Item::Function {
//             name: "foo".into(),
//             params: vec![
//                 Binding {
//                     mutable: true,
//                     name: "a".into(),
//                     type_annotation: None
//                 },
//                 Binding {
//                     mutable: false,
//                     name: "b".into(),
//                     type_annotation: Some(Type::Ident {
//                         name: "Int".into(),
//                         generics: vec![]
//                     })
//                 }
//             ],
//             return_type: None,
//             body: Expr::BinaryOp {
//                 op: Bop::Add,
//                 lhs: Expr::Ident("a".into()).into(),
//                 rhs: Expr::Ident("b".into()).into()
//             }
//         }
//     )
// }

// #[test]
// fn parse_file() {
//     let items = parse_ast(
//         r#"
//         fn wow_we_did_it(mut x, bar: Bar<Baz<T>, U>): fn(Int): Int -> {
//             let mut x: (Float, T) = -7.0 + sin(y);
//             x = if (bar < 3) {
//                 let baz = bar.value + 2 * 4;
//                 x + 1;
//             } else if (bar <= 2)
//                 fizz(3, 5.1)
//         }  

//         struct Foo<T, U> {
//             x: Str,
//             bar: Bar<Baz<T>, [U]>,
//         }
//     "#,
//     );

//     assert_eq!(
//         items[0],
//         Item::Function {
//             name: "wow_we_did_it".into(),
//             params: vec![
//                 Binding {
//                     mutable: true,
//                     name: "x".into(),
//                     type_annotation: None
//                 },
//                 Binding {
//                     mutable: false,
//                     name: "bar".into(),
//                     type_annotation: Some(Type::Ident {
//                         name: "Bar".into(),
//                         generics: vec![
//                             Type::Ident {
//                                 name: "Baz".into(),
//                                 generics: vec![Type::Ident {
//                                     name: "T".into(),
//                                     generics: vec![],
//                                 }],
//                             },
//                             Type::Ident {
//                                 name: "U".into(),
//                                 generics: vec![],
//                             }
//                         ],
//                     })
//                 }
//             ],
//             return_type: Some(Type::Fn {
//                 params: vec![Type::Ident {
//                     name: "Int".into(),
//                     generics: vec![]
//                 }],
//                 result: Type::Ident {
//                     name: "Int".into(),
//                     generics: vec![]
//                 }
//                 .into()
//             }),
//             body: Expr::Block {
//                 exprs: vec![
//                     Expr::Let {
//                         binding: Binding {
//                             mutable: true,
//                             name: "x".into(),
//                             type_annotation: Some(Type::Tuple(vec![
//                                 Type::Ident {
//                                     name: "Float".into(),
//                                     generics: vec![]
//                                 },
//                                 Type::Ident {
//                                     name: "T".into(),
//                                     generics: vec![]
//                                 }
//                             ]))
//                         },
//                         value: Expr::BinaryOp {
//                             op: Bop::Add,
//                             lhs: Expr::UnaryOp {
//                                 op: Unop::Neg,
//                                 expr: Expr::Float(7.0)
//                             }
//                             .into(),
//                             rhs: Expr::FnCall {
//                                 fun: Expr::Ident("sin".into()).into(),
//                                 args: vec![Expr::Ident("y".into())]
//                             }
//                             .into()
//                         }
//                         .into()
//                     },
//                     Expr::BinaryOp {
//                         op: Bop::Assign,
//                         lhs: Expr::Ident("x".into()).into(),
//                         rhs: Expr::If {
//                             cond: Expr::BinaryOp {
//                                 op: Bop::Lt,
//                                 lhs: Expr::Ident("bar".into()).into(),
//                                 rhs: Expr::Int(3)
//                             }
//                             .into(),
//                             th: Expr::Block {
//                                 exprs: vec![
//                                     Expr::Let {
//                                         binding: Binding {
//                                             mutable: false,
//                                             name: "baz".into(),
//                                             type_annotation: None
//                                         },
//                                         value: Expr::BinaryOp {
//                                             op: Bop::Add,
//                                             lhs: Expr::FieldAccess {
//                                                 base: Expr::Ident("bar".into()).into(),
//                                                 field: "value".into()
//                                             }
//                                             .into(),
//                                             rhs: Expr::BinaryOp {
//                                                 op: Bop::Mul,
//                                                 lhs: Expr::Int(2),
//                                                 rhs: Expr::Int(4)
//                                             }
//                                             .into()
//                                         }
//                                         .into()
//                                     },
//                                     Expr::BinaryOp {
//                                         op: Bop::Add,
//                                         lhs: Expr::Ident("x".into()).into(),
//                                         rhs: Expr::Int(1)
//                                     }
//                                 ],
//                                 trailing: false
//                             }
//                             .into(),
//                             el: Some(
//                                 Expr::If {
//                                     cond: Expr::BinaryOp {
//                                         op: Bop::Leq,
//                                         lhs: Expr::Ident("bar".into()).into(),
//                                         rhs: Expr::Int(2)
//                                     }
//                                     .into(),
//                                     th: Expr::FnCall {
//                                         fun: Expr::Ident("fizz".into()).into(),
//                                         args: vec![Expr::Int(3).into(), Lit::Float(5.1)]
//                                     }
//                                     .into(),
//                                     el: None
//                                 }
//                                 .into()
//                             )
//                         }
//                         .into()
//                     },
//                 ],
//                 trailing: true
//             }
//         }
//     );

//     assert_eq!(
//         items[1],
//         Item::Struct {
//             name: "Foo".into(),
//             generic_params: vec!["T".into(), "U".into(),],
//             fields: vec![
//                 Field {
//                     name: "x".into(),
//                     ty: Type::Ident {
//                         name: "Str".into(),
//                         generics: vec![],
//                     },
//                 },
//                 Field {
//                     name: "bar".into(),
//                     ty: Type::Ident {
//                         name: "Bar".into(),
//                         generics: vec![
//                             Type::Ident {
//                                 name: "Baz".into(),
//                                 generics: vec![Type::Ident {
//                                     name: "T".into(),
//                                     generics: vec![],
//                                 }],
//                             },
//                             Type::Array(
//                                 Type::Ident {
//                                     name: "U".into(),
//                                     generics: vec![],
//                                 }
//                                 .into()
//                             )
//                         ],
//                     },
//                 }
//             ]
//         }
//     );
// }
