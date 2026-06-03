use std::fmt::format;

use crate::{AST::ASTNode, DataType};

// ============================================================================
// DEVELOPMENT NOTE:
// Initial logic for this system was drafted using AI pseudocode.
// The entire codebase has since been manually rewritten, refactored, and
// engineered from scratch. Future development is entirely human-written.
// ============================================================================

pub fn transpile_to_c(ast_nodes: Vec<ASTNode>) -> String {
    let mut c_code_output = String::new();

    c_code_output.push_str("#include <stdio.h>\n\n");
    c_code_output.push_str("#include <stdbool.h>\n\n");
    c_code_output.push_str("int main() {\n");

    for node in ast_nodes {
        let c_line = convert_node_to_c_string(&node);

        if c_line.ends_with(';') || c_line.ends_with('{') {
            c_code_output.push_str(&format!("   {}\n", c_line));
        } else {
            c_code_output.push_str(&format!("   {};\n", c_line));
        }
    }

    c_code_output.push_str("    return 0;\n");
    c_code_output.push_str("}\n");

    c_code_output
}

fn convert_node_to_c_string(node: &ASTNode) -> String {
    match node {
        ASTNode::VariableDecleration {
            name,
            value,
            var_type,
        } => {
            let value_text = convert_node_to_c_string(&*value);

            match var_type {
                DataType::VarType::Int => format!("int {} = {}", name, value_text),
                DataType::VarType::Float => format!("float {} = {}", name, value_text),
                DataType::VarType::Bool => format!("bool {} = {}", name, value_text),
            }
        }

        ASTNode::PrintDecleration { target } => {
            let target_text = convert_node_to_c_string(&*target);
            format!("printf(\"%d\\n\", {})", target_text)
        }

        ASTNode::Identifier { name } => name.clone(),

        ASTNode::LiteralInt { value } => value.to_string(),

        ASTNode::BinaryOperaion {
            left,
            operator,
            right,
        } => {
            let left_text = convert_node_to_c_string(&*left);
            let right_text = convert_node_to_c_string(&*right);
            format!("({} {} {})", left_text, operator, right_text)
        }

        ASTNode::LiteralBool { value } => {
            if *value {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }

        ASTNode::Assingment { name, value } => {
            let val_text = convert_node_to_c_string(&*value);
            format!("{} = {}", name, val_text)
        }

        _ => panic!(
            "Transpiler Error: Node type not implemented for transpilation: {:?}",
            node
        ),
    }
}
