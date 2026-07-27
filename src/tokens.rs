use strum_macros::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Display)]
pub enum Tokens {
    Identifier(String),
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralString(String),

    CONST,
    VAR,
    PRINT,
    FUNC,
    IF,

    OperatorAssign,    // '='
    OperatorSpark,     // ':-'
    OperatorAdd,       // '+'
    OperatorSubtract,  // '-'
    OperatorMultiply,  // '*'
    OperatorDivide,    // '/'
    OperatorSet,       // the : to set stuff
    OperatorSemicolon, // ';'

    UnaryOperatorNot, //'!'

    OperatorEqual,
    OperatorNotEqual,
    OperatorLess,
    OperatorLessEqual,
    OperatorGreater,
    OperatorGreaterEqual,

    OperatorAnd,
    OperatorOr,

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

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Negate,
    Positive,
    Not,
}

#[derive(Clone, Debug)]
pub enum ComparisonOperator {
    Compare,
    NotCompare,
    GreaterThan,
    GreaterThanEqual,
    SmallerThan,
    SmallerThanEqual,
}

#[derive(Clone, Debug)]
pub enum LogicalOperator {
    And,
    Or,
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

impl TryFrom<&Tokens> for UnaryOperator {
    type Error = ();

    fn try_from(token: &Tokens) -> Result<Self, Self::Error> {
        match token {
            Tokens::OperatorAdd => Ok(UnaryOperator::Positive),
            Tokens::OperatorSubtract => Ok(UnaryOperator::Negate),
            Tokens::UnaryOperatorNot => Ok(UnaryOperator::Not),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Tokens> for ComparisonOperator {
    type Error = ();

    fn try_from(token: &Tokens) -> Result<Self, Self::Error> {
        match token {
            Tokens::OperatorEqual => Ok(ComparisonOperator::Compare),
            Tokens::OperatorNotEqual => Ok(ComparisonOperator::NotCompare),
            Tokens::OperatorGreater => Ok(ComparisonOperator::GreaterThan),
            Tokens::OperatorGreaterEqual => Ok(ComparisonOperator::GreaterThanEqual),
            Tokens::OperatorLess => Ok(ComparisonOperator::SmallerThan),
            Tokens::OperatorLessEqual => Ok(ComparisonOperator::SmallerThanEqual),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Tokens> for LogicalOperator {
    type Error = ();

    fn try_from(token: &Tokens) -> Result<Self, Self::Error> {
        match token {
            Tokens::OperatorAnd => Ok(LogicalOperator::And),
            Tokens::OperatorOr => Ok(LogicalOperator::Or),
            _ => Err(()),
        }
    }
}
