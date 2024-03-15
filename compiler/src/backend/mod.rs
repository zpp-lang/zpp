pub mod lexer;
pub mod parser;

mod tests {
    use crate::backend::lexer::Lexer;
    use crate::backend::parser::Parser;

    #[test]
    pub fn test_backend() {
        let mut lexer = Lexer::new("test.rs", "func T() { printLn(\"Hello, world!\") }");
        let mut tokens = lexer.tokenize();

        for token in &tokens {
            println!("{:?}", token)
        }

        let mut parser = Parser::new("test.rs", tokens);
        let ast = parser.parse();
    }
}