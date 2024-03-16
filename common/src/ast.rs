use std::collections::HashMap;
use codespan::Span;

#[derive(Debug, PartialEq)]
pub enum Type {
    Reference,
    Void,
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
