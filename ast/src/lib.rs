pub mod lexer;
pub mod parser;

use std::collections::HashMap;

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
    ReferenceVal(String),
    StringVal(String),
    IntVal(i32),
    FloatVal(f32),
    BoolVal(bool),
    NullVal,
}

/// Represents a node in the abstract syntax tree (AST). Each node is given a unique
/// span that represents the starting position and ending position of the evaluated
/// expression.
#[derive(Debug, PartialEq)]
pub enum AstNode {
    /// AST node type that represents an empty evaluation
    Empty,
    /// AST node type that represents a Z++ document
    Document {
        body: Box<AstNode> // Typically a 'Block' ast node
    },
    /// AST node type that represents a collection of other AST Nodes
    Block {
        /// Usually consists of expressions or assignments, however when the Block belongs to a
        /// Document, it can contain functions as well.
        body: Vec<AstNode>
    },
    /// AST node type that represents an identifier or reference
    Identifier {
        /// This is the name of the reference or type identifier.
        name: String,
    },
    /// AST node type that represents a basic function call
    FunctionCall {
        name: String,
        param_list: Vec<TypedValue>,
    },
    /// AST node type representing a function prototype
    FuncDeclaration {
        name: String,
        params: HashMap<String, Type>,
        returns: Type,
        body: Box<AstNode>,
    },
    /// AST node type representing a variable declaration
    VarDeclaration {
        name: String,
        is_mutable: bool, // TODO add interpreter support for mutability
        is_static: bool,
        value: TypedValue,
    },
    /// AST node type representing a variable assignment
    VarAssignment {
        name: String,
        new_value: TypedValue // type will get checked upon evaluation
    },
}