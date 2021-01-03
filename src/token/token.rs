use crate::lexer::symbols::{FromString, Keyword, Operator, Punctuation};
use std::convert::From;

#[derive(Debug, Clone)]
pub enum TokenType {
    Keyword(Keyword),
    Punctuation(Punctuation),
    Identifier(String),
    Operator(Operator),
    None, //DataType e.g. Bool, Int, String, Float etc
}

impl From<String> for TokenType {
    fn from(s: String) -> Self {
        let k = Keyword::from_string(s.clone());
        match k {
            Some(kw) => TokenType::Keyword(kw),
            None => match Punctuation::from_string(s.clone()) {
                Some(pc) => TokenType::Punctuation(pc),
                None => match Operator::from_string(s.clone()) {
                    Some(op) => TokenType::Operator(op),
                    None => TokenType::Identifier(s),
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
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
