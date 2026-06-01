use std::fmt::format;

use crate::AST::ASTNode;

pub fn transpile_to_c(ast_nodes: Vec<ASTNode>) -> String {
    let mut c_code_output = String::new();

    c_code_output.push_str("#include <stdio.h>\n\n");
    c_code_output.push_str("int main() {\n");

    for node in ast_nodes {
        let c_line = convert_node_to_c_string(&node);
        c_code_output.push_str(&format!("    {}\n", c_line));
    }

    c_code_output.push_str("    return 0;\n");
    c_code_output.push_str("}\n");

    c_code_output
}

fn convert_node_to_c_string(node: &ASTNode) -> String {
    match node {
        ASTNode::VariableDecleration { name, value } => {
            let value_text = convert_node_to_c_string(&*value);

            format!("int {} = {};", name, value_text)
        }

        ASTNode::PrintDecleration { target } => {
            let target_text = convert_node_to_c_string(&*target);
            format!("printf(\"%d\\n\", {});", target_text)
        },

        ASTNode::Identifier { name } => name.clone(),

        ASTNode::LiteralInt{ value} => {value.to_string()},
        _ => panic!("Error at transpilation"),
    }
}
