use crate::lexer::symbols::{FromString, Keyword, Punctuation};
use std::convert::From;

#[derive(Debug)]
enum TokenType {
    Keyword(Keyword),
    Punctuation(Punctuation),
    Identifier(String),
    //DataType e.g. Bool, Int, String, Float etc
}

impl From<String> for TokenType {
    fn from(s: String) -> Self {
        let k = Keyword::from_string(s.clone());
        match k {
            Some(kw) => TokenType::Keyword(kw),
            None => match Punctuation::from_string(s.clone()) {
                Some(pc) => TokenType::Punctuation(pc),
                None => TokenType::Identifier(s),
            },
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    pub(crate) fn new<S: Into<String>>(input: S) -> Self {
        let s = input.into();
        return Token {
            token_type: TokenType::from(s.clone()),
            value: s.clone(),
        };
    }
}
