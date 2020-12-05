use crate::token::token::Token;

pub fn lex(input: String) -> Vec<Token> {
    let input = input.trim().to_lowercase();

    let tokens = input
        .split(" ")
        .map(|input| Token::new(input))
        .collect::<Vec<Token>>();

    println!("{:?}", tokens);

    return tokens;
}
