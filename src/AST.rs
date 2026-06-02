#[derive(Debug, Clone)]

// ============================================================================
// DEVELOPMENT NOTE:
// Initial logic for this system was drafted using AI pseudocode.
// The entire codebase has since been manually rewritten, refactored, and
// engineered from scratch. Future development is entirely human-written.
// ============================================================================
pub enum ASTNode {
    VariableDecleration {
        name: String,
        value: Box<ASTNode>, //Makes shure that there can't be a memory overflow!
    },

    PrintDecleration {
        target: Box<ASTNode>,
    },

    BinaryOperaion {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },

    Identifier {
        name: String,
    },

    LiteralInt {
        value: i64,
    },

    LiteralFloat {
        value: f64,
    },

    LiteralBool {
        value: bool,
    }
}
