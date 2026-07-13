use strum_macros::Display;

use crate::{DataType, tokens::BinaryOperator};

#[derive(Debug, Clone)]
#[warn(non_snake_case)]
#[allow(dead_code)]

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

    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },

    PrintDecleration {
        target: Box<ASTNode>,
    },

    BinaryOperation {
        left: Box<ASTNode>,
        operator: BinaryOperator,
        right: Box<ASTNode>,
    },

    Assignment {
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

    ExpressionStatement {
        expression: Box<ASTNode>,
    },
    NONE,
}

#[derive(Debug, Clone, Display)]
#[allow(dead_code)]
pub enum OperatorType {
    Addtion,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone)]
#[warn(non_snake_case)]
#[allow(dead_code)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: DataType::VarType,
}
