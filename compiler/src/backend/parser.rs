use std::collections::HashMap;
use codespan::Span;
use crate::backend::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Type {
    Reference,
    String,
    Int,
    Float,
    Boolean,
    Null,
}

#[derive(Debug, PartialEq)]
pub enum TypedValue {
    IdentVal(String),
    StringVal(String),
    IntVal(i64),
    FloatVal(f64),
    BoolVal(bool),
    NullVal,
}

/// Represents a node in the abstract syntax tree (AST). Each node is given a unique
/// span that represents the starting position and ending position of the evaluated
/// expression.
///
/// The code generator (frontend) will implement a function called 'codegen' for
/// each node, which will emit an AST node as LLVM IR.
#[derive(Debug, PartialEq)]
pub enum AstNode<'a> {
    /// AST node type that represents an empty evaluation
    Empty,
    /// AST node type that represents a Z++ document
    Document {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        file_name: &'a str, // The name of the file
        body: Box<AstNode<'a>> // Typically a 'Block' ast node
    },
    /// AST node type that represents a collection of other AST Nodes
    Block {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        /// Usually consists of expressions or assignments, however when the Block belongs to a
        /// Document, it can contain functions as well.
        body: Vec<AstNode<'a>>
    },
    /// AST node type that represents an identifier or reference
    Identifier {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        /// This is the name of the reference or type identifier.
        name: String,
    },
    /// AST node type that represents a basic function call
    FunctionCall {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        name: String,
        param_list: Vec<TypedValue>,
    },
    /// AST node type representing a function prototype
    FuncDeclaration {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        name: String,
        params: HashMap<String, Type>,
        returns: Type,
        body: Box<AstNode<'a>>,
    },
    /// AST node type representing a variable declaration
    VarDeclaration {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        name: String,
        is_mutable: bool, // TODO add compiler support for mutability
        is_static: bool,
        value: TypedValue,
    },
    /// AST node type representing a variable assignment
    VarAssignment {
        /// This is used to keep track of the entire expression, for emitting warnings,
        /// errors, and other useful information.
        span: Span,
        name: String,
        new_value: TypedValue // type will get checked upon evaluation
    },
}

/// Parses tokens into an abstract syntax tree (AST)
pub struct Parser<'a> {
    file_name: &'a str,
    tokens: Vec<Token>
}

impl<'a> Parser<'a> {
    /// Creates a new instance of the Parser.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file being parsed.
    /// * `tokens` - The tokens to parse into an AST.
    pub fn new(file_name: &'a str, tokens: Vec<Token>) -> Self {
        Parser {
            file_name,
            tokens
        }
    }

    pub fn parse(&mut self) -> AstNode {
        AstNode::Document {
            span: Default::default(),
            file_name: self.file_name,
            body: Box::new(AstNode::Empty)
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
    fn check_allow_reassignment(&self, declaring_node: AstNode<'a>, assigning_node: AstNode<'a>) -> bool {
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
    fn get_type_for(&self, value: TypedValue) -> Type {
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