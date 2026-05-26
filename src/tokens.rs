#[derive(Debug)]
pub enum Tokens {
    Identifier(String),
    LiteralInt(i32),

    CONST,
    VAR,
    PRINT,

    OperatorAssign,
    OperatorSpark,
    OperatorAdd,
    OperatorSubtract,
    OperatorMultioply,
    OperatorDivide,

    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,

    ERROR,
    EOF,
}
