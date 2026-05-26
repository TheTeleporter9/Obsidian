use crate::ast::{ASTArena, ASTNode};
use crate::token::Token;
use std::collections::HashMap;

pub struct VariableSymbol {
    pub type_name: String,
    pub is_mutable: bool,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, VariableSymbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    // Push a new local scope level onto the stack (e.g., when entering a function)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Pop the current local scope off the stack (e.g., when leaving a function)
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn insert(&mut self, name: String, type_name: String, is_mutable: bool) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(
                name,
                VariableSymbol {
                    type_name,
                    is_mutable,
                },
            );
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&VariableSymbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}

pub struct TypeChecker<'a> {
    arena: &'a ASTArena,
    symbol_table: SymbolTable,
}

impl<'a> TypeChecker<'a> {
    pub fn new(arena: &'a ASTArena) -> Self {
        Self {
            arena,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn check_program(&mut self, root_ids: &[usize]) {
        for &id in root_ids {
            self.check_node(id);
        }
    }

    pub fn check_node(&mut self, id: usize) -> String {
        let node = self.arena.get(id);

        match node {
            ASTNode::IntLiteral(_) => String::from("int"),
            ASTNode::FloatLiteral(_) => String::from("float"),
            ASTNode::BoolLiteral(_) => String::from("bool"),
            ASTNode::StringLiteral(_) => String::from("string"),

            ASTNode::Identifier(name) => {
                if let Some(symbol) = self.symbol_table.lookup(name) {
                    symbol.type_name.clone()
                } else {
                    panic!("Type Error: Use of undeclared identifier '{}'", name);
                }
            }

            ASTNode::BinaryOp {
                operator,
                left,
                right,
            } => {
                let left_type = self.check_node(*left);
                let right_type = self.check_node(*right);

                match operator {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                        if left_type == "int" && right_type == "int" {
                            String::from("int")
                        } else if left_type == "float" && right_type == "float" {
                            String::from("float")
                        } else {
                            panic!(
                                "Type Error: Cannot apply math operator '{:?}' to mismatched types '{}' and '{}'",
                                operator, left_type, right_type
                            );
                        }
                    }
                    _ => panic!("Type Error: Unsupported binary operator configuration"),
                }
            }

            ASTNode::VariableDecleration {
                name,
                is_mutable,
                initializer,
            } => {
                let init_type = self.check_node(*initializer);
                self.symbol_table
                    .insert(name.clone(), init_type.clone(), *is_mutable);
                String::from("void")
            }

            ASTNode::SparkFeedPipeline {
                input,
                function_name,
            } => {
                let input_type = self.check_node(*input);
                if function_name == "calculate" {
                    if input_type == "int" {
                        String::from("int")
                    } else {
                        panic!("Type Error: 'calculate' pipeline expects 'int' type data input");
                    }
                } else {
                    input_type
                }
            }

            ASTNode::PrintStatement { value } => {
                let _value_type = self.check_node(*value);
                String::from("void")
            }

            ASTNode::FunctionDeclaration {
                name: _,
                params: _,
                return_type: _,
                body,
            } => {
                for &stmt_id in body {
                    self.check_node(stmt_id);
                }
                String::from("void")
            }

            ASTNode::ReturnStatement { value } => {
                if let Some(&val_idx) = value.as_ref() {
                    self.check_node(val_idx);
                }
                String::from("void")
            }
        }
    }
}
