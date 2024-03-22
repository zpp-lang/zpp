use std::process::exit;
use codespan::{ByteIndex, Span};
use logos::Logos;
use common::errors::{create_span, Reporting};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
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
    #[regex("[0-9]+(\\.[0-9]+)?", |lex| lex.slice().parse::<f32>().expect("Unknown error"))]
    IntLiteral(f32),
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    reporter: Reporting<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(name: &'a str, input: &'a str) -> Self {
        Lexer {
            input,
            reporter: Reporting::new(name, input)
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
                } else if let Err(_) = res {
                    errors.push(self.reporter.emit_error(
                        "An error occurred while lexing".to_string(),
                        create_span(lexer.span()),
                        vec![
                            "You might be running an outdated version of the interpreter!".to_string()
                        ],
                        "E001".to_string()
                    ))
                }
            } else {
                break
            }
        }

        if errors.len() > 0 {
            exit(1)
        }

        tokens
    }
}