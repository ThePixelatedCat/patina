use super::{Type, TypeChecker};
use crate::parser::Parser;

#[test]
fn typecheck_block() {
    let input = "
    {
        let mut y: Int = 5;
        3 + 1 - 2;
        y = 256;
        if (y < 3) {
            let a = -5;
            a
        } else 32;
    }";
    let mut parser = Parser::new(input);
    let expr = parser.expression().unwrap();
    //let types = TypeChecker::default().type_of(&expr).unwrap();

    //assert_eq!(types, Type::Tuple(vec![]));
}
