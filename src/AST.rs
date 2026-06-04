use crate::DataType;

#[derive(Debug, Clone)]
#[warn(non_snake_case)]

// ============================================================================
// DEVELOPMENT NOTE:
// Initial logic for this system was drafted using AI pseudocode.
// The entire codebase has since been manually rewritten, refactored, and
// engineered from scratch. Future development is entirely human-written.
// ============================================================================
pub enum ASTNode {
    VariableDecleration {
        name: String,
        var_type: DataType::VarType,
        value: Box<ASTNode>, //Makes shure that there can't be a memory overflow!
    },

    FunctionDecleration {
        name: String,
        return_type: DataType::VarType,
        parameters: Vec<FunctionParameter>,
        body: Vec<ASTNode>,
        return_value: Box<ASTNode>,
    },

    PrintDecleration {
        target: Box<ASTNode>,
    },

    BinaryOperaion {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },

    Assingment {
        name: String,
        value: Box<ASTNode>,
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
    },

    NONE,
}

#[derive(Debug, Clone)]
#[warn(non_snake_case)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: DataType::VarType,
}
