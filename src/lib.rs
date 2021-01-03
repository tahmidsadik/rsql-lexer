#![allow(dead_code)]
mod lexer;
mod token;

use lexer::lex::lex;

#[cfg(test)]
mod tests {
    use super::lex;
    use crate::lexer::lex::tokenize;

    // #[test]
    // fn lex_sanity() {
    //     let input_str = "select * from orders".to_string();
    //
    //     assert_eq!(lex(input_str).len(), 4);
    // }

    #[test]
    fn test_tokenize() {
        let input_str = "select name from table";

        let (_, tokens) = tokenize(input_str, vec![]);
        println!("tokens = {:?}", tokens);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
