#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Int,
    Void,
    Return,
    // Literals
    Identifier(String),
    IntLiteral(i32),

    // Single char tokens
    OpenParenthesis,
    ClosedParenthesis,
    Semicolon,
    OpenBraces,
    ClosedBraces,

    // End of File
    EOF,
}
