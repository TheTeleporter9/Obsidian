#[derive(Debug, PartialEq, Clone)]
// ============================================================================
// DEVELOPMENT NOTE:
// Initial logic for this system was drafted using AI pseudocode.
// The entire codebase has since been manually rewritten, refactored, and
// engineered from scratch. Future development is entirely human-written.
// ============================================================================
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
    OperatorMultiply,
    OperatorDivide,
    OperatorSet, // the : to set stuff
    OperatorSemicolon,

    BracketOpen, //{}
    BracketClose,
    BraceOpen, //()
    BraceClose,
    SquareBracketOpen, //[]
    SquareBracketClose,
    Comma,

    TypeInt,
    TypeString,
    TypeFloat,
    TypeBoolean,

    OptionTrue,
    OptionFalse,

    ERROR,
    EOF,
}
