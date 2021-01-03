// use logos::Logos;

// #[derive(Logos, Debug, PartialEq)]
// enum NumberToken {
//     // skipping whitespaces
//     #[error]
//     #[regex(r"[ \t\n\f]+", logos::skip)]
//     Error,
//
//     #[regex(r"[0-9]+\.[0-9]+")]
//     Number,
// }

fn isDigit(c: char) -> bool {
    c.is_digit(10)
}

#[test]
fn lex_num() {}
