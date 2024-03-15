use codespan::{ByteIndex, Span};
use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{Config, emit};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n]+")]
pub enum TokenType {
    // Keywords
    #[token("func")]
    Func,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,

    // Literals
    #[regex("[0-9]+(\\.[0-9]+)?", |lex| lex.slice().parse::<f64>().expect("Unknown error"))]
    IntLiteral(f64),
    #[regex("true|false", |lex| lex.slice() == "true")]
    BoolLiteral(bool),
    #[regex(r#""[^"\\\r\n]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLiteral(String),
    #[token("null")]
    NullLiteral,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Operators and symbols
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Slash,
    #[token("=")]
    Eq,
    #[token("==")]
    DoubleEq,
    #[token("!")]
    Not,
    #[token("!=")]
    NotEq,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(">=")]
    GreaterEq,
    #[token("<=")]
    LessEq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("@")]
    At,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    span: Span,
}

pub struct Lexer<'a> {
    file: SimpleFile<&'a str, &'a str>,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(name: &'a str, input: &'a str) -> Self {
        Lexer {
            file: SimpleFile::new(name, input),
            input
        }
    }
    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut lexer = TokenType::lexer(self.input);

        loop {
            if let Some(res) = lexer.next() {
                if let Ok(token) = res {
                    tokens.push(Token {
                        token_type: token,
                        span: Span::new(
                            ByteIndex::from(lexer.span().start as u32), // starting character position
                            ByteIndex::from(lexer.span().end as u32), // ending character position
                        )
                    });
                } else if let Err(err) = res {
                    errors.push(Diagnostic::error()
                        .with_labels(vec![
                            Label::new(
                                LabelStyle::Primary,
                                (),
                                lexer.span().start..lexer.span().end
                            ).with_message("Invalid token")
                        ])
                        .with_message("An error occurred while lexing".to_string())
                        .with_code("001")
                        .with_notes(vec![
                            "You might be running an outdated version of the compiler!".to_string()
                        ])
                    )
                }
            } else {
                break
            }
        }

        if errors.len() > 0 {
            for error in errors {
                let writer = StandardStream::stderr(ColorChoice::Auto);
                let config = Config::default();
                emit(&mut writer.lock(), &config, &self.file, &error)
                    .expect("Unknown error");
            }
        }

        tokens
    }
}