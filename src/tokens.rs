#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Identifier(String),
    LiteralInt(i64),

    CONST,
    VAR,
    PRINT,
    FUNC,

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

    TypeInt,
    TypeString,

    ERROR,
    EOF,
}
