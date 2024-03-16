pub mod lexer;
pub mod parser;

mod tests {
    use crate::backend::parser::Parser;

    #[test]
    pub fn test_backend() {
        let mut parser = Parser::new("test.rs", "func T() { printLn(\"Hello, world!\") }");
        let ast = parser.parse();
    }
}