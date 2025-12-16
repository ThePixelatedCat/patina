use super::{Parser, Token, ast::Stmt};

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn statement(&mut self) -> Stmt {
        match self.peek() {
            Token::Let => {
                self.consume(Token::Let.ty());
                let mutable = self.at(Token::Mut.ty());
                if mutable {
                    self.consume(Token::Mut.ty());
                }

                let Some(Token::Ident(ident)) = self.next() else {
                    panic!("expected identifier after let")
                };

                let type_annotation = self.at(Token::Colon.ty()).then(|| {
                    self.consume(Token::Colon.ty());
                    let Some(Token::Ident(ty)) = self.next() else {
                        panic!("expected type after colon")
                    };
                    ty
                });

                self.consume(Token::Eq.ty());
                let value = self.expression();
                self.consume(Token::Semicolon.ty());

                Stmt::Let {
                    mutable,
                    ident,
                    type_annotation,
                    value,
                }
            }
            Token::Ident(_) => {
                let Token::Ident(ident) = self.next().unwrap() else {
                    unreachable!()
                };
                self.consume(Token::Eq.ty());
                let value = self.expression();
                self.consume(Token::Semicolon.ty());
                Stmt::Assign { ident, value }
            }
            _ => {
                let expr = self.expression();
                if self.at(Token::Semicolon.ty()) {
                    self.consume(Token::Semicolon.ty());
                    Stmt::Expr(expr)
                } else {
                    panic!("expected semicolon after expression")
                }
            }
        }
    }
}
