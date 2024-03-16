use std::collections::HashMap;
use std::iter::Peekable;
use std::process::exit;
use std::slice::Iter;
use codespan::{ByteIndex, Span};
use codespan_reporting::diagnostic::Diagnostic;
use common::errors::{merge_span, Reporting};
use crate::backend::lexer::{Lexer, Token, TokenType};
use common::ast::{AstNode, Type};

/// Parses tokens into an abstract syntax tree (AST)
pub struct Parser<'a> {
    file_name: &'a str,
    tokens: Vec<Token>,
    reporter: Reporting<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new instance of the Parser.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file being parsed.
    /// * `tokens` - The tokens to parse into an AST.
    pub fn new(file_name: &'a str, contents: &'a str) -> Self {
        let lexer = Lexer::new(file_name, contents);
        let tokens = lexer.tokenize();

        Parser {
            file_name,
            tokens,
            reporter: Reporting::new(file_name, contents),
        }
    }

    pub fn parse(&mut self) -> AstNode {
        let mut iter = self.tokens.iter().peekable();
        let mut ast = Vec::new();
        let mut errors = Vec::new();

        while let Some(token) = iter.next() {
            match &token.token_type {
                // case for when the current token is a function declaration
                TokenType::Func => {
                    if let Some(next) = iter.next() {
                        // check if the next token type is an identifier
                        if let TokenType::Identifier(name) = &next.token_type {
                            let func_name = name.clone(); // dereference the func name

                            // enter the parameters list
                            if let Some(possible_lparen) = iter.next() { // check if there is a new token after the identifier
                                if let TokenType::LParen = &possible_lparen.token_type { // check if it's a parenthesis or not
                                    // we know it's a valid function declaration, so we call eval_fn_params
                                    let params_result = self.eval_fn_params(&mut iter);
                                    // handle the result if an error was found
                                    if let Err(report) = params_result {
                                        errors.push(report);
                                        break
                                    }

                                    // params have been declared successfully, let's get the function body
                                    // TODO parse function type
                                    if let Some(possible_lbrace) = iter.next() { // check if there are more tokens
                                        if let TokenType::LBrace = &possible_lbrace.token_type { // check the next token type
                                            // enter the function body and wrap everything up
                                            let body_result = self.eval_fn_body(&mut iter);
                                            // handle the result if an error was found
                                            if let Err(report) = body_result {
                                                errors.push(report);
                                                break
                                            }

                                            // valid function declaration, emit ast node
                                            let params = params_result.unwrap();
                                            let body = body_result.unwrap();

                                            ast.push(AstNode::FuncDeclaration {
                                                span: merge_span(&next.span, &body.1.span),
                                                name: func_name,
                                                params: params.0,
                                                returns: Type::Void,
                                                body: Box::new(body.0),
                                            })
                                        } else { // throw an error, missing lbrace
                                            errors.push(self.reporter.emit_error(
                                                format!("Expected brace after function parameters, got {:?}", &possible_lbrace.token_type),
                                                possible_lbrace.span,
                                                vec![],
                                                "E003".to_string()
                                            ));
                                            break
                                        }
                                    } else { // no more tokens, so an error is thrown
                                        errors.push(self.reporter.emit_error(
                                            "Expected brace for function declaration body".to_string(),
                                            possible_lparen.span,
                                            vec![],
                                            "E002".to_string()
                                        ));
                                        break
                                    }
                                } else { // throw an error, missing parenthesis
                                    errors.push(self.reporter.emit_error(
                                        format!("Expected parenthesis after function identifier, got {:?}", &possible_lparen.token_type),
                                        possible_lparen.span,
                                        vec![],
                                        "E003".to_string()
                                    ));
                                    break
                                }
                            } else { // if there isn't we throw an error
                                errors.push(self.reporter.emit_error(
                                    "Expected parenthesis for function declaration".to_string(),
                                    next.span,
                                    vec![],
                                    "E002".to_string()
                                ));
                                break
                            }
                        } else { // if it isn't, we report an error
                            errors.push(self.reporter.emit_error(
                                format!("Expected identifier for function declaration, got {:?}", &next.token_type),
                                next.span,
                                vec![],
                                "E003".to_string()
                            ));
                            break
                        }
                    } else { // if there isn't another token, we report an error
                        errors.push(self.reporter.emit_error(
                            "Expected identifier for function declaration".to_string(),
                            token.span,
                            vec![],
                            "E002".to_string()
                        ));
                        break
                    }
                },

                // case for when the token is unknown to the parser, but known to the lexer
                _ => ()
            }
        }

        if errors.len() > 0 {
            exit(1)
        }

        AstNode::Document {
            file_name: self.file_name,
            body: Box::new(AstNode::Block {
                span: Span::new(
                    (&self.tokens).first().unwrap().span.start(),
                    (&self.tokens).last().unwrap().span.end(),
                ),
                body: ast,
            })
        }
    }

    fn eval_fn_body(
        &self,
        tokens: &mut Peekable<Iter<'a, Token>>
    ) -> Result<(AstNode<'a>, Token), Diagnostic<()>> {
        // TODO implement
        tokens.next();
        Ok((AstNode::Empty, Token {
            token_type: TokenType::Func,
            span: Span::new(
                ByteIndex::from(1),
                ByteIndex::from(52)
            ),
        }))
    }

    /// Evaluates the function parameters for the current Function Declaration. It takes
    /// a mutable reference to the Tokens as a peekable, and will move the iterator forward
    /// as it traverses through the lex result.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A mutable reference to a Peekable list of tokens.
    ///
    /// # Returns
    ///
    /// The list of function parameters accompanied by the current
    /// token, or an error emitted by a Reporter.
    fn eval_fn_params(
        &self,
        tokens: &mut Peekable<Iter<'a, Token>>
    ) -> Result<(HashMap<String, Type>, Token), Diagnostic<()>> {
        // TODO implement
        tokens.next();
        Ok((HashMap::new(), Token {
            token_type: TokenType::Func,
            span: Span::new(
                ByteIndex::from(0),
                ByteIndex::from(5)
            ),
        }))
    }
}