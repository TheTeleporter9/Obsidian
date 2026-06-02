use crate::{AST::ASTNode, DataType, token_table::variable_table};

fn ensure_type(node: &ASTNode, expected: DataType::VarType, table: &variable_table) {
    match node {
        ASTNode::Identifier { name } => {
            let actual = table
                .get_type_by_name(name)
                .expect("Semantic Error: Vairblae not declared");
            if actual != expected {
                panic!("Type Mismatch: Expected {:?}, but got an Integer", expected);
            }
        }

        //TODO add recursive check for bin ops.
        _ => {}
    }
}
