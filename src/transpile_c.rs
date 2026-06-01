use crate::AST::ASTNode;


fn transpile_to_C(ast_nodes: Vec<ASTNode>) {
    let mut c_code_output : Vec<String> = Vec::new();

    //MAIN BLOCK FOR C
    c_code_output.push("#include <stdio.h>".to_string());

    //all variable decleration


    c_code_output.push("int main{\n".to_string());

    for node in ast_nodes {
        let c_line = convert_node_to_c_string(node);

        c_code_output.push("");
    }
    
}

fn convert_node_to_c_string(node: ASTNode) {

}