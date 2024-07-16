#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),

    // Operators
    Assign, // =
    Plus,   // +

    // Delimeters
    Comma,     // ,
    Semicolon, // ;
    Lparen,    // (
    Rparen,    // )
    Lcurly,    // {
    Rcurly,    // }

    // Keywords
    Function, // fn
    Let,      // let
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        _ => Token::Ident(ident.to_string()),
    }
}
