use strum_macros::Display;

use crate::{
    AST, DataType,
    tokens::{BinaryOperator, ComparisonOperator, LogicalOperator, UnaryOperator},
};

#[derive(Debug, Clone)]
#[warn(non_snake_case)]
#[allow(dead_code)]

pub enum ASTNode {
    VariableDeclaration {
        name: String,
        var_type: DataType::VarType,
        value: Box<ASTNode>, //Makes shure that there can't be a memory overflow!
    },

    FunctionDeclaration {
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

    PrintStatement {
        target: Box<ASTNode>,
    },

    IfStatment {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
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

    UnaryOerator {
        operator: UnaryOperator,
        operand: Box<ASTNode>,
    },

    ComparisonOperator {
        left: Box<ASTNode>,
        operator: ComparisonOperator,
        right: Box<ASTNode>,
    },

    LogicalOperation {
        left: Box<ASTNode>,
        operator: LogicalOperator,
        right: Box<ASTNode>,
    },
}

#[derive(Debug, Clone)]
#[warn(non_snake_case)]
#[allow(dead_code)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: DataType::VarType,
}
