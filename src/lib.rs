#![allow(dead_code)]
mod lexer;
mod token;

use lexer::lex::lex;

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn lex_sanity() {
        let input_str = "select * from orders".to_string();

        assert_eq!(lex(input_str).len(), 4);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
