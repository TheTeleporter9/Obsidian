use std::fmt::format;

use crate::{
    AST::ASTNode,
    DataType::{self, VarType},
    semantic::symbol_table,
    tokens::{BinaryOperator, ComparisonOperator, LogicalOperator},
};

pub fn transpile_to_c(ast_nodes: Vec<ASTNode>) -> String {
    let mut function_code = String::new();
    let mut main_code = String::new();

    for node in ast_nodes {
        match node {
            ASTNode::FunctionDeclaration { .. } => {
                function_code.push_str(&convert_node_to_c_string(&node));
                function_code.push_str("\n\n");
            }

            _ => {
                main_code.push_str("    ");
                main_code.push_str(&convert_node_to_c_string(&node));
                main_code.push_str(";\n");
            }
        }
    }

    let mut output = String::new();

    output.push_str("#include <stdio.h>\n");
    output.push_str("#include <stdbool.h>\n\n");

    output.push_str(&function_code);

    output.push_str("int main() {\n");
    output.push_str(&main_code);
    output.push_str("    return 0;\n");
    output.push_str("}\n");

    output
}

fn operator_to_c(op: &BinaryOperator) -> &'static str {
    match op {
        BinaryOperator::Add => "+",
        BinaryOperator::Subtract => "-",
        BinaryOperator::Multiply => "*",
        BinaryOperator::Divide => "/",
        BinaryOperator::Assign => "=",
    }
}

fn type_to_c(var_type: &VarType) -> &'static str {
    match var_type {
        VarType::Int => "int",
        VarType::Float => "float",
        VarType::Bool => "bool",
    }
}
fn convert_node_to_c_string(node: &ASTNode) -> String {
    match node {
        ASTNode::VariableDeclaration {
            name,
            value,
            var_type,
        } => {
            let value_text = convert_node_to_c_string(&*value);

            format!("{} {} = {}", type_to_c(var_type), name, value_text)
        }

        ASTNode::PrintStatement { target } => {
            let format_specifier = match target.as_ref() {
                ASTNode::LiteralInt { .. } => "%d",
                ASTNode::LiteralFloat { .. } => "%f",
                ASTNode::LiteralBool { .. } => "%d",

                ASTNode::Identifier { name } => match symbol_table::get_symbol(name).unwrap() {
                    VarType::Int => "%d",
                    VarType::Float => "%f",
                    VarType::Bool => "%d",
                },

                _ => "%d", // TODO: Determine type of expressions
            };

            let target_text = convert_node_to_c_string(target);

            format!("printf(\"{}\\n\", {})", format_specifier, target_text)
        }

        ASTNode::Identifier { name } => name.clone(),

        ASTNode::LiteralInt { value } => value.to_string(),

        ASTNode::LiteralFloat { value } => value.to_string(),

        ASTNode::BinaryOperation {
            left,
            operator,
            right,
        } => {
            let left_text = convert_node_to_c_string(&*left);
            let right_text = convert_node_to_c_string(&*right);
            format!("({} {} {})", left_text, operator_to_c(operator), right_text)
        }

        ASTNode::LiteralBool { value } => {
            if *value {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }

        ASTNode::Assignment { name, value } => {
            let val_text = convert_node_to_c_string(&*value);
            format!("{} = {}", name, val_text)
        }

        ASTNode::FunctionDeclaration {
            name,
            return_type,
            parameters,
            body,
            return_value,
        } => {
            let c_return_type = match return_type {
                DataType::VarType::Int => "int",
                DataType::VarType::Float => "float",
                DataType::VarType::Bool => "bool",
            };

            let params = parameters
                .iter()
                .map(|p| {
                    let param_type = match p.param_type {
                        DataType::VarType::Int => "int",
                        DataType::VarType::Float => "float",
                        DataType::VarType::Bool => "bool",
                    };

                    format!("{} {}", param_type, p.name)
                })
                .collect::<Vec<_>>()
                .join(", ");

            let mut function_code = format!("{} {}({}) {{\n", c_return_type, name, params);

            for statement in body {
                function_code.push_str("    ");
                function_code.push_str(&convert_node_to_c_string(statement));
                function_code.push_str(";\n");
            }

            function_code.push_str(&format!(
                "    return {};\n",
                convert_node_to_c_string(return_value)
            ));

            function_code.push_str("}");

            function_code
        }
        ASTNode::FunctionCall { name, arguments } => {
            let args = arguments
                .iter()
                .map(|arg| convert_node_to_c_string(arg))
                .collect::<Vec<String>>()
                .join(", ");

            format!("{}({})", name, args)
        }

        ASTNode::ComparisonOperator {
            left,
            operator,
            right,
        } => {
            let left_text = convert_node_to_c_string(left);
            let right_text = convert_node_to_c_string(right);

            let op = match operator {
                ComparisonOperator::Compare => "==",
                ComparisonOperator::NotCompare => "!=",
                ComparisonOperator::GreaterThan => ">",
                ComparisonOperator::GreaterThanEqual => ">=",
                ComparisonOperator::SmallerThan => "<",
                ComparisonOperator::SmallerThanEqual => "<=",
            };

            format!("({} {} {})", left_text, op, right_text)
        }

        ASTNode::ExpressionStatement { expression } => convert_node_to_c_string(expression),

        ASTNode::LogicalOperation {
            left,
            operator,
            right,
        } => {
            let left_text = convert_node_to_c_string(left);
            let right_text = convert_node_to_c_string(right);

            let op = match operator {
                LogicalOperator::And => "&&",
                LogicalOperator::Or => "||",
            };

            format!("({} {} {})", left_text, op, right_text)
        }

        _ => panic!(
            "Transpiler Error: Node type not implemented for transpilation: {:?}",
            node
        ),
    }
}
