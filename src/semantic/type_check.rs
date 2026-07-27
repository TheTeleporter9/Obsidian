use crate::{
    AST::ASTNode,
    DataType::VarType,
    tokens::{BinaryOperator, UnaryOperator},
};

use crate::semantic::symbol_table;

use std::collections::HashMap;

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

fn check_expression(node: &ASTNode) -> VarType {
    match node {
        ASTNode::LiteralInt { .. } => VarType::Int,
        ASTNode::LiteralFloat { .. } => VarType::Float,
        ASTNode::LiteralBool { .. } => VarType::Bool,

        ASTNode::Identifier { name } => symbol_table::get_symbol(name)
            .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),

        ASTNode::UnaryOperator { operator, operand } => {
            let operand_type = check_expression(operand);

            match operator {
                UnaryOperator::Negate => match operand_type {
                    VarType::Int | VarType::Float => operand_type,
                    _ => panic!("Unary '-' requires a number."),
                },

                UnaryOperator::Positive => match operand_type {
                    VarType::Int | VarType::Float => operand_type,
                    _ => panic!("Unary '+' requires a number."),
                },

                UnaryOperator::Not => {
                    if operand_type != VarType::Bool {
                        panic!("Unary '!' requires a bool.");
                    }
                    VarType::Bool
                }
            }
        }

        ASTNode::BinaryOperation {
            left,
            operator,
            right,
        } => check_binary_operation(left, operator, right),

        _ => panic!("Expected expression."),
    }
}

fn check_assignment(node: &ASTNode) {
    let ASTNode::Assignment { name, value } = node else {
        unreachable!();
    };

    let variable_type =
        symbol_table::get_symbol(name).unwrap_or_else(|| panic!("Undefined variable '{}'", name));

    let value_type = check_expression(value);

    if variable_type != value_type {
        panic!(
            "Type Error: Cannot assign {:?} to variable '{}' of type {:?}",
            value_type, name, variable_type
        );
    }
}

fn check_print(node: &ASTNode) {
    let ASTNode::PrintStatement { target } = node else {
        unreachable!();
    };

    check_expression(target);
}

fn check_binary_operation(left: &ASTNode, operator: &BinaryOperator, right: &ASTNode) -> VarType {
    let left_type = check_expression(left);
    let right_type = check_expression(right);

    match operator {
        BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide => {
            if left_type != right_type {
                panic!(
                    "Type Error: Cannot perform {:?} on {:?} and {:?}",
                    operator, left_type, right_type
                );
            }

            match left_type {
                VarType::Int | VarType::Float => left_type,
                _ => panic!("Arithmetic operators require numeric operands."),
            }
        }
        BinaryOperator::Assign => unreachable!("Parser should have produced ASTNode::Assignment"),
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

    symbol_table::insert_symbol(name.clone(), var_type);
    if *var_type != value_type {
        panic!(
            "Type Error: Vairable '{}' is declared as {:?}, but received {:?}",
            name, var_type, value_type
        )
    }
}
