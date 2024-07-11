use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    EOF,
    Number,
    String,
    Identifier,

    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,

    Colon,
    Comma,
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        Token { kind, value }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.value)
    }
}
