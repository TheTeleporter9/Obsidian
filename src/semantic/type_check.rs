use crate::{
    AST::{self, ASTNode::{self, LiteralInt}}, DataType::{self, VarType},
};

use std::collections::HashMap;

let mut symbol_table = HashMap::<String, VarType>::new();

pub fn check_program(nodes: &[ASTNode]) {
    println!("_________________Type Checker_______________________");
    for node in nodes {
        check_statement(node);
    }
}

fn check_statement(node: &ASTNode) {
    match node {
        ASTNode::VariableDeclaration { .. } => {
            check_variable_declaration(node);
        }

        ASTNode::Assignment { .. } => {
            check_assignment(node);
        }

        ASTNode::PrintStatement { .. } => {
            check_print(node);
        }

        ASTNode::ExpressionStatement { .. } => {
            check_expression(node);
        }

        _ => panic!("Unexpected statement."),
    }
}

fn check_expression(node: &ASTNode) -> DataType::VarType {
    match node {
        ASTNode::LiteralInt { .. } => return DataType::VarType::Int,
        ASTNode::LiteralFloat { .. } => return DataType::VarType::Float,
        ASTNode::LiteralBool { .. }=> return DataType::VarType::Bool,
        ASTNode::Identifier { .. } => 
    }
}




fn check_variable_declaration(node: &ASTNode) {
    let ASTNode::VariableDeclaration {
        name,
        var_type,
        value,
    } = node
    else {
        unreachable!();
    };

    println!("{}", name);

    let value_type = check_expression(value);
    symbol_table.insert();
    if *var_type != value_type {
        panic!(
            "Type Error: Vairable '{}' is declared as {:?}, but received {:?}",
            name, var_type, value_type
        )
    }
}
