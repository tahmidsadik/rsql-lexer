use crate::lexer::symbols::{FromString, Operator, Punctuation};
use crate::token::token::{Token, TokenType};

pub fn lex(input: String) -> Vec<Token> {
    let input = input.trim().to_lowercase();

    let tokens = input
        .split(" ")
        .map(|input| Token::new(input))
        .collect::<Vec<Token>>();

    println!("{:?}", tokens);

    return tokens;
}

pub fn tokenize(input: &str, mut tokens: Vec<Token>) -> (&str, Vec<Token>) {
    // let mut next_input = input;
    let mut next_input = "";
    if input.len() == 0 {
        return ("", (*tokens).to_vec());
    }

    for (idx, c) in input.chars().enumerate() {
        if Punctuation::from_string((&c).to_string()).is_some()
            || Operator::from_string((&c).to_string()).is_some()
        {
            // println!(
            //     "current idx = {}, current character = {}, punc from string = {:?}, Operator from string = {:?}",
            //     idx,
            //     &c,
            //     Punctuation::from_string((&c).to_string()),
            //     Operator::from_string((&c).to_string())
            // );
            tokens.push(Token::new(&input[0..idx])); // taking characters upto this index
            tokens.push(Token::new((&c).to_string())); // taking the current char
            next_input = if idx + 1 < input.len() {
                &input[(idx + 1)..input.len()]
            } else {
                ""
            };
            break;
        }
    }

    println!("next input = {}", next_input,);

    tokenize(&next_input, tokens)
}
