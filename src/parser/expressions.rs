use std::mem::discriminant as dis;

use super::{
    Parser, Token,
    ast::{Bop, Expr, Lit, Unop},
};

trait PrefixOperator {
    fn binding_power(&self) -> u8;
}

trait InfixOperator {
    fn binding_power(&self) -> (u8, u8);
}

impl PrefixOperator for Unop {
    fn binding_power(&self) -> u8 {
        match self {
            Unop::Neg | Unop::Not => 51,
        }
    }
}

impl InfixOperator for Bop {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Bop::Or => (1, 2),
            Bop::And => (3, 4),
            Bop::Eqq | Bop::Neq => (5, 6),
            Bop::Gt | Bop::Lt | Bop::Leq | Bop::Geq => (7, 8),
            Bop::Add | Bop::Sub => (9, 10),
            Bop::Mul | Bop::Div => (11, 12),
            Bop::Exp => (22, 21), // <- This binds stronger to the left!
        }
    }
}

impl<'input, I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn expression(&mut self) -> Expr {
        self.parse_expression(0)
    }

    pub fn parse_expression(&mut self, binding_power: u8) -> Expr {
        let mut lhs = match self.peek() {
            Token::LParen => {
                self.consume(dis(&Token::LParen));
                let expr = self.parse_expression(0);
                self.consume(dis(&Token::RParen));
                expr
            }
            Token::IntLit(_)
            | Token::FloatLit(_)
            | Token::StringLit(_)
            | Token::CharLit(_)
            | Token::True
            | Token::False => {
                let lit = match self.next().unwrap() {
                    Token::IntLit(int) => Lit::Int(int),
                    Token::FloatLit(float) => Lit::Float(float),
                    Token::StringLit(string) => Lit::Str(string),
                    Token::CharLit(char) => Lit::Char(char),
                    Token::True => Lit::Bool(true),
                    Token::False => Lit::Bool(false),
                    _ => unreachable!(),
                };
                Expr::Literal(lit)
            }
            Token::Ident(_) => {
                let Some(Token::Ident(ident)) = self.next() else {
                    unreachable!()
                };
                Expr::Ident(ident)
            }
            // Token::If => {
            //     self.consume(dis(&Token::If));
            //     self.consume(dis(&Token::LParen));
            //     let cond = self.parse_expression(0);
            //     self.consume(dis(&Token::RParen));
            //     let th = self.parse_expression(0);
            //     self.consume(dis(&Token::Else));
            //     let el = self.parse_expression(0);

            //     Expr::If {
            //         cond: Box::new(cond),
            //         th: Box::new(th),
            //         el: Box::new(el),
            //     }
            // }
            // Token::LBrace => {
            //     let mut exprs = Vec::new();
            //     while self.peek() != &Token::RBrace {
            //         exprs.push(self.parse_expression(0));
            //     }
            //     self.consume(dis(&Token::RBrace));
            //     Expr::Block(exprs)
            // }
            op @ (Token::Minus | Token::Bang) => {
                let dis = dis(op);
                let op = match op {
                    Token::Minus => Unop::Neg,
                    Token::Bang => Unop::Not,
                    _ => unreachable!(),
                };
                self.consume(dis);
                let right_binding_power = op.binding_power();
                let expr = self.parse_expression(right_binding_power);
                Expr::UnaryOp {
                    op,
                    expr: Box::new(expr),
                }
            }
            token => panic!("Unknown start of expression: `{:?}`", token),
        };
        loop {
            let token = self.peek();
            let op = match token {
                Token::Plus => Bop::Add,
                Token::Minus => Bop::Sub,
                Token::Times => Bop::Mul,
                Token::FSlash => Bop::Div,
                Token::UpArrow => Bop::Exp,
                Token::Eqq => Bop::Eqq,
                Token::Neq => Bop::Neq,
                Token::And => Bop::And,
                Token::Or => Bop::Or,
                Token::LAngle => Bop::Lt,
                Token::Leq => Bop::Leq,
                Token::RAngle => Bop::Gt,
                Token::Geq => Bop::Geq,
                Token::Eof => break,
                Token::RParen | Token::RBrace | Token::Comma | Token::Semicolon => break,
                token => panic!("Unknown operator: `{:?}`", token),
            };

            let (left_binding_power, right_binding_power) = op.binding_power();

            if left_binding_power < binding_power {
                break;
            }

            let dis = dis(token);
            self.consume(dis);

            let rhs = self.parse_expression(right_binding_power);
            lhs = Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        lhs
    }
}
