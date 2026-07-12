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

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
}

impl TryFrom<&Tokens> for BinaryOperator {
    type Error = ();

    fn try_from(token: &Tokens) -> Result<Self, Self::Error> {
        match token {
            Tokens::OperatorAdd => Ok(BinaryOperator::Add),
            Tokens::OperatorSubtract => Ok(BinaryOperator::Subtract),
            Tokens::OperatorMultiply => Ok(BinaryOperator::Multiply),
            Tokens::OperatorDivide => Ok(BinaryOperator::Divide),
            Tokens::OperatorAssign => Ok(BinaryOperator::Assign),
            _ => Err(()),
        }
    }
}
