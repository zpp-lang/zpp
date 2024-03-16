use codespan::{Span};
use codespan_reporting::files::SimpleFile;
use common::errors::Reporting;
use crate::backend::lexer::{Lexer, Token};
use common::ast::{AstNode, TypedValue, Type};

/// Parses tokens into an abstract syntax tree (AST)
pub struct Parser<'a> {
    file: SimpleFile<&'a str, &'a str>,
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
        let mut lexer = Lexer::new(file_name, contents);
        let mut tokens = lexer.tokenize();

        Parser {
            file: SimpleFile::new(file_name, contents),
            file_name,
            tokens,
            reporter: Reporting::new(file_name, contents)
        }
    }

    pub fn parse(&mut self) -> AstNode {
        let mut iter = self.tokens.iter().peekable();
        let mut ast = Vec::new();

        while let Some(token) = iter.next() {
            match &token.token_type {
                // case for when the token is unknown to the parser, but known to the lexer
                _ => {
                    self.reporter.emit_warning(
                        format!("Unrecognized token {:?}", token.token_type),
                        token.span,
                        vec![
                            "If you are using a development version of the compiler, you may dismiss this warning. If not, file a bug report!".to_string()
                        ],
                        "PAR1".to_string(),
                    );
                }
            }
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

    /// Checks whether reassignment of a variable is allowed based on its type.
    ///
    /// # Arguments
    ///
    /// * `declaring_node` - The AST node representing the variable declaration.
    /// * `assigning_node` - The AST node representing the variable assignment.
    ///
    /// # Returns
    ///
    /// Returns true if reassignment is allowed, false otherwise.
    fn check_allow_reassignment(&self, declaring_node: &AstNode<'a>, assigning_node: &AstNode<'a>) -> bool {
        return match (declaring_node, assigning_node) {
            (AstNode::VarDeclaration { is_mutable, value, .. },
                AstNode::VarAssignment { new_value, .. }) => {
                if !is_mutable {
                    return false
                }

                let orig_type = self.get_type_for(value);
                let changed_type = self.get_type_for(new_value);

                if orig_type == changed_type {
                    return true
                }

                false
            },

            _ => false
        }
    }

    /// Retrieves the type of a specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The typed value for which to retrieve the type.
    ///
    /// # Returns
    ///
    /// Returns the type of the value.
    fn get_type_for(&self, value: &TypedValue) -> Type {
        return match value {
            TypedValue::IdentVal(_) => Type::Reference,
            TypedValue::StringVal(_) => Type::String,
            TypedValue::IntVal(_) => Type::Int,
            TypedValue::FloatVal(_) => Type::Float,
            TypedValue::BoolVal(_) => Type::Boolean,
            TypedValue::NullVal => Type::Null
        }
    }
}