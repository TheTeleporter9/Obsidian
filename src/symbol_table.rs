use std::iter::Map;

use crate::AST::ASTNode;

pub struct SymbolTable {
    table: Map<String, ASTNode>,
}

struct VariableInformation {}
