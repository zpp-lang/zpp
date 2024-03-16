pub mod lexer;
pub mod parser;

mod tests {
    #[test]
    pub fn test_backend() {
        let mut parser = crate::backend::parser::Parser::new("test.rs", "func MyTestFunction() { \nprintLn(\"Hello, world!\") \n}");
        let ast = parser.parse();

        println!("{:?}", ast)
    }
}