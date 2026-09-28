use std::fmt;

#[derive(Debug, Clone)]
pub struct ParserError(pub String);

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser Error: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct LexerError(pub String);

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexer Error: {}", self.0)
    }
}


#[derive(Debug, Clone)]
pub struct CodegenError(pub String);

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Codegen Error: {}", self.0)
    }
}

impl From<std::io::Error> for CodegenError {
    fn from(value: std::io::Error) -> Self {
        CodegenError(value.to_string())
    }
}