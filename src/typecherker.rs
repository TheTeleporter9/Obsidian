use crate::ast::{ASTArena, ASTNode};
use crate::token::Token;
use std::collections::HashMap;
use std::fmt;

// Error definitions for various type-checking failures
#[derive(Debug)]
pub enum TypeError {
    UndeclaredIdentifier { name: String },
    MismatchedBinaryOp { operator: Token, left: String, right: String },
    UnsupportedOperator,
    InvalidPipelineInput { pipeline: String, expected: String, found: String },
}

// Formats error variants into user-friendly compiler messages with error codes
impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::UndeclaredIdentifier { name } => {
                write!(f, "[Error E001]: Use of undeclared identifier '{}'", name)
            }
            TypeError::MismatchedBinaryOp { operator, left, right } => {
                write!(f, "[Error E002]: Cannot apply operator '{:?}' to mismatched types '{}' and '{}'", operator, left, right)
            }
            TypeError::UnsupportedOperator => {
                write!(f, "[Error E003]: Unsupported binary operator configuration")
            }
            TypeError::InvalidPipelineInput { pipeline, expected, found } => {
                write!(f, "[Error E004]: Pipeline function '{}' expects input of type '{}', but found '{}'", pipeline, expected, found)
            }
        }
    }
}

// Holds data tracking an individual variable's metadata
pub struct VariableSymbol {
    pub type_name: String,
    pub is_mutable: bool,
}

// Manages scoped variable storage using a stack of hash maps
pub struct SymbolTable {
    scopes: Vec<HashMap<String, VariableSymbol>>,
}

impl SymbolTable {
    // Creates table initialized with a global scope layer
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    // Enters a new nesting level (e.g., block or function)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Exits the current nesting level, dropping its local variables
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    // Stores a variable symbol inside the current innermost scope
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

    // Searches outward from inner scopes to global scope to find a variable
    pub fn lookup(&self, name: &str) -> Option<&VariableSymbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}

// Engine that traverses the AST to validate types
pub struct TypeChecker<'a> {
    arena: &'a ASTArena, // Borrowed reference to the central AST storage
    symbol_table: SymbolTable,
}

impl<'a> TypeChecker<'a> {
    // Binds the type checker to the lifetime of the input AST arena
    pub fn new(arena: &'a ASTArena) -> Self {
        Self {
            arena,
            symbol_table: SymbolTable::new(),
        }
    }

    // Starts verification process across sequence of root nodes
    pub fn check_program(&mut self, root_ids: &[usize]) {
        for &id in root_ids {
            self.check_node(id);
        }
    }

    // Resolves and processes types recursively for any given node ID
    pub fn check_node(&mut self, id: usize) -> String {
        let node = self.arena.get(id);

        match node {
            // Literals immediately evaluate to their respective primitive type names
            ASTNode::IntLiteral(_) => String::from("int"),
            ASTNode::FloatLiteral(_) => String::from("float"),
            ASTNode::BoolLiteral(_) => String::from("bool"),
            ASTNode::StringLiteral(_) => String::from("string"),

            // Looks up variable type or fails if it is missing from current scope
            ASTNode::Identifier(name) => {
                if let Some(symbol) = self.symbol_table.lookup(name) {
                    symbol.type_name.clone()
                } else {
                    panic!("{}", TypeError::UndeclaredIdentifier { name: name.clone() });
                }
            }

            // Validates that both operands match and support the math operator
            ASTNode::BinaryOp { operator, left, right } => {
                let left_type = self.check_node(*left);
                let right_type = self.check_node(*right);

                match operator {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                        if left_type == "int" && right_type == "int" {
                            String::from("int")
                        } else if left_type == "float" && right_type == "float" {
                            String::from("float")
                        } else {
                            panic!("{}", TypeError::MismatchedBinaryOp {
                                operator: operator.clone(),
                                left: left_type,
                                right: right_type
                            });
                        }
                    }
                    _ => panic!("{}", TypeError::UnsupportedOperator),
                }
            }

            // Typchecks initializer value and saves variable to current scope
            ASTNode::VariableDecleration { name, is_mutable, initializer } => {
                let init_type = self.check_node(*initializer);
                self.symbol_table.insert(name.clone(), init_type, *is_mutable);
                String::from("void")
            }

            // Special node type validating data stream requirements for pipelines
            ASTNode::SparkFeedPipeline { input, function_name } => {
                let input_type = self.check_node(*input);
                if function_name == "calculate" {
                    if input_type == "int" {
                        String::from("int")
                    } else {
                        panic!("{}", TypeError::InvalidPipelineInput {
                            pipeline: function_name.clone(),
                            expected: String::from("int"),
                            found: input_type
                        });
                    }
                } else {
                    input_type
                }
            }

            // Checks that inner expression to be printed is valid
            ASTNode::PrintStatement { value } => {
                let _value_type = self.check_node(*value);
                String::from("void")
            }

            // Creates isolated local scope and type-checks function body statements
            ASTNode::FunctionDeclaration { name: _, params: _, return_type: _, body } => {
                self.symbol_table.push_scope();
                for &stmt_id in body {
                    self.check_node(stmt_id);
                }
                self.symbol_table.pop_scope();
                String::from("void")
            }

            // Typechecks expression being passed out by the return statement
            ASTNode::ReturnStatement { value } => {
                if let Some(&val_idx) = value.as_ref() {
                    self.check_node(val_idx);
                }
                String::from("void")
            }
        }
    }
}