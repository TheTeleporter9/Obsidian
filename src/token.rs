#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // =========================================================
    // 1. Keywords (from spec §31)
    // =========================================================
    Package,
    Import,

    Const,
    Var,

    Fn,
    Return,

    Class,
    Impl,
    Interface,
    Enum,

    SelfLower, // `self`
    SelfUpper, // `Self`

    Pub,
    Priv,
    Mut,

    If,
    Else,
    For,
    While,
    Loop,
    Break,
    Continue,
    Match,

    Fail,
    Pass,
    Guard,
    Try,
    Catch,

    Defer,
    Alloc,
    Free,

    Async,
    Await,
    Spawn,
    Scope,

    Comptime,

    True,
    False,
    None,
    Some,

    Void,
    Never,
    Any,

    In,
    Or,
    Step,

    Print,

    // =========================================================
    // 2. Identifiers & Literals
    // =========================================================
    Identifier(String),

    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    CharLiteral(char),

    // =========================================================
    // 3. Operators (spec §28 + §30)
    // =========================================================
    Assign,  // =
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Power,   // **

    Equal,     // ==
    NotEqual,  // !=
    Less,      // <
    Greater,   // >
    LessEq,    // <=
    GreaterEq, // >=

    And,  // &&
    OrOp, // ||
    Not,  // !

    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    BitNot, // ~

    Shl, // <<
    Shr, // >>

    // =========================================================
    // 4. Obsidian-specific operators
    // =========================================================
    SparkFeed,     // -:
    Arrow,         // ->
    FatArrow,      // =>
    Range,         // ..
    RangeIncl,     // ..=
    OptionalChain, // ?.
    Assert,        // !!
    Namespace,     // ::
    Concat,        // <>

    // =========================================================
    // 5. Delimiters
    // =========================================================
    LParen,
    RParen, // ( )
    LBrace,
    RBrace, // { }
    LBracket,
    RBracket, // [ ]

    Comma,
    Dot,
    Semicolon,
    Colon,

    // =========================================================
    // 6. Special
    // =========================================================
    EOF,
    Unknown(char),
}
