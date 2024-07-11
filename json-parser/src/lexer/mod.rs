mod token;

use token::{Token, TokenKind};

type PatternHandler = dyn Fn(&mut Lexer, String);

struct Pattern {
    pattern: String,
    handler: Box<PatternHandler>,
}

impl Pattern {
    fn new(pattern: String, handler: Box<PatternHandler>) -> Self {
        Pattern { pattern, handler }
    }
}

struct Lexer {
    patterns: Vec<Pattern>,
    tokens: Vec<Token>,
    source: String,
    pos: usize,
}

impl Lexer {
    fn new(source: String) -> Self {
        Lexer {
            pos: 0,
            source,
            tokens: vec![],
            patterns: vec![
                Pattern::new(
                    String::from("{"),
                    default_handler(TokenKind::OpenCurly, String::from("{")),
                ),
                Pattern::new(
                    String::from("{"),
                    default_handler(TokenKind::OpenCurly, String::from("{")),
                ),
            ],
        }
    }

    pub fn advance_n(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn at_eof(&self) -> bool {
        self.pos >= self.source.len()
    }
}

fn default_handler(kind: TokenKind, value: String) -> Box<PatternHandler> {
    Box::new(move |lex: &mut Lexer, pattern: String| {
        lex.advance_n(pattern.len());
        lex.push(Token::new(kind, value.to_owned()));
    })
}

pub fn tokenize(source: String) -> Vec<Token> {
    let lex = Lexer::new(source);

    while !lex.at_eof() {
        let mut matched = false;

        for p in &lex.patterns {}
    }

    todo!()
}
