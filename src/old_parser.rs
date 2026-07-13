// use std::mem::transmute;
//
// use crate::AST::{ASTNode, FunctionParameter};
// use crate::DataType;
// use crate::token_table::{self};
// use crate::tokens::Tokens::{self, Identifier};
//
// // ============================================================================
// // DEVELOPMENT NOTE:
// // Initial logic for this system was drafted using AI pseudocode.
// // The entire codebase has since been manually rewritten, refactored, and
// // engineered from scratch. Future development is entirely human-written.
// // ============================================================================
// pub struct Parser {
//     pub tokens: Vec<Tokens>,
//     position: usize,
//     variable_table: token_table::variable_table,
//     function_table: token_table::FunctionTable,
// }
//
// impl Parser {
//     pub fn new(input_tokens: Vec<Tokens>) -> Self {
//         Self {
//             tokens: input_tokens,
//             position: 0,
//             variable_table: token_table::variable_table::new(),
//             function_table: token_table::FunctionTable::new(),
//         }
//     }
//
//     //Helper functions
//
//     fn advance(&mut self) {
//         if self.position <= self.tokens.len() {
//             self.position += 1;
//         }
//     }
//
//     pub fn parse(&mut self) -> Vec<ASTNode> {
//         let mut program_nodes = Vec::new();
//
//         while self.position < self.tokens.len() {
//             println!("{:?}", &self.tokens[self.position]);
//
//             let node = match &self.tokens[self.position] {
//                 Tokens::VAR => self.parse_variable_declaration(),
//                 Tokens::PRINT => self.parse_print_statement(),
//                 Tokens::FUNC => self.parse_function_decleration(),
//                 Tokens::LiteralInt(_) => self.parse_expression(),
//                 Tokens::Identifier(_) => {
//                     if self.position + 1 < self.tokens.len()
//                         && self.tokens[self.position + 1] == Tokens::OperatorAssign
//                     {
//                         self.parse_assigment() // Return the node, don't push it yet!
//                     } else {
//                         self.parse_expression() // Return the node
//                     }
//                 }
//                 _ => panic!("Unexpected Token! {:?}", &self.tokens[self.position]),
//             };
//
//             program_nodes.push(node);
//         }
//
//         program_nodes
//     }
//
//     fn parse_variable_declaration(&mut self) -> ASTNode {
//         self.advance(); // Skip VAR
//
//         let var_type = match &self.tokens[self.position] {
//             Tokens::TypeInt => DataType::VarType::Int,
//             Tokens::TypeFloat => DataType::VarType::Float,
//             Tokens::TypeBoolean => DataType::VarType::Bool,
//             _ => panic!("invalid type"),
//         };
//
//         self.advance();
//
//         //Get vairable name
//         let var_name = match &self.tokens[self.position] {
//             Tokens::Identifier(name) => name.clone(),
//             _ => panic!("Expected a valid variable name!"),
//         };
//
//         self.advance(); // Skip identifier
//
//         match &self.tokens[self.position] {
//             Tokens::OperatorAssign => self.advance(),
//             _ => panic!("Expected an '=' at variable declaration"),
//         }
//
//         let value_node = self.parse_expression();
//         if var_type == DataType::VarType::Bool {
//             self.validate_bool_assignment(&value_node);
//         }
//
//         token_table::variable_table::add_variable_reference(
//             &mut self.variable_table,
//             ASTNode::VariableDecleration {
//                 name: var_name.clone(),
//                 var_type: var_type,
//                 value: Box::new(value_node.clone()),
//             },
//         );
//
//         ASTNode::VariableDecleration {
//             name: var_name,
//             var_type: var_type,
//             value: Box::new(value_node),
//         }
//     }
//
//     pub fn parse_function_decleration(&mut self) -> ASTNode {
//         self.advance(); // skip FUNC
//
//         let return_type = match &self.tokens[self.position] {
//             Tokens::TypeInt => DataType::VarType::Int,
//             Tokens::TypeFloat => DataType::VarType::Float,
//             Tokens::TypeBoolean => DataType::VarType::Bool,
//             _ => panic!("Expected return type"),
//         };
//
//         self.advance();
//
//         let function_name = match &self.tokens[self.position] {
//             Tokens::Identifier(name) => name.clone(),
//             _ => panic!("Expected function name"),
//         };
//
//         self.advance();
//
//         match &self.tokens[self.position] {
//             Tokens::BracketOpen => self.advance(),
//             _ => panic!("Expected '{{'"),
//         }
//
//         let parameters = self.parse_parameter_list();
//
//         self.variable_table.push_scope();
//
//         for parameter in &parameters {
//             self.variable_table
//                 .add_variable_reference(ASTNode::VariableDecleration {
//                     name: parameter.name.clone(),
//                     var_type: parameter.param_type,
//                     value: Box::new(ASTNode::NONE),
//                 });
//         }
//
//         let mut body = Vec::new();
//
//         while self.position < self.tokens.len()
//             && !matches!(self.tokens[self.position], Tokens::BracketClose)
//         {
//             body.push(self.parse_statement());
//         }
//
//         if matches!(self.tokens[self.position], Tokens::BracketClose) {
//             self.advance();
//         } else {
//             panic!("Expected '}}'");
//         }
//
//         match &self.tokens[self.position] {
//             Tokens::BraceOpen => self.advance(),
//             _ => panic!("Expected '('"),
//         }
//
//         let return_value = self.parse_expression();
//
//         match &self.tokens[self.position] {
//             Tokens::BraceClose => self.advance(),
//             _ => panic!("Expected ')'"),
//         }
//
//         self.variable_table.pop_scope();
//
//         let node = ASTNode::FunctionDecleration {
//             name: function_name,
//             return_type,
//             parameters,
//             body,
//             return_value: Box::new(return_value),
//         };
//
//         self.function_table.add_function_reference(node.clone());
//
//         node
//     }
//     pub fn parse_parameter_list(&mut self) -> Vec<FunctionParameter> {
//         let mut parameters = Vec::new();
//
//         // Expect '['
//         match &self.tokens[self.position] {
//             Tokens::SquareBracketOpen => self.advance(),
//             _ => panic!("Expected '[' to begin parameter list"),
//         }
//
//         // Handle empty parameter list: []
//         if matches!(self.tokens[self.position], Tokens::SquareBracketClose) {
//             self.advance();
//             return parameters;
//         }
//
//         loop {
//             // Parse parameter type
//             let param_type = match &self.tokens[self.position] {
//                 Tokens::TypeInt => {
//                     self.advance();
//                     DataType::VarType::Int
//                 }
//                 Tokens::TypeFloat => {
//                     self.advance();
//                     DataType::VarType::Float
//                 }
//                 Tokens::TypeBoolean => {
//                     self.advance();
//                     DataType::VarType::Bool
//                 }
//                 _ => panic!("Expected parameter type"),
//             };
//
//             // Parse parameter name
//             let param_name = match &self.tokens[self.position] {
//                 Tokens::Identifier(name) => {
//                     let name = name.clone();
//                     self.advance();
//                     name
//                 }
//                 _ => panic!("Expected parameter name"),
//             };
//
//             parameters.push(FunctionParameter {
//                 name: param_name,
//                 param_type,
//             });
//
//             // Either ',' or ']'
//             match &self.tokens[self.position] {
//                 Tokens::Comma => {
//                     self.advance();
//                 }
//
//                 Tokens::SquareBracketClose => {
//                     self.advance();
//                     break;
//                 }
//
//                 _ => panic!("Expected ',' or ']' in parameter list"),
//             }
//         }
//
//         parameters
//     }
//
//     fn parse_function_call(&mut self) -> ASTNode {
//         let function_name = match &self.tokens[self.position] {
//             Tokens::Identifier(name) => name.clone(),
//             _ => panic!("Expected function name"),
//         };
//
//         self.advance(); // consume function name
//
//         match &self.tokens[self.position] {
//             Tokens::SquareBracketOpen => self.advance(),
//             _ => panic!("Expected '['"),
//         }
//
//         let mut arguments = Vec::new();
//
//         while !matches!(self.tokens[self.position], Tokens::SquareBracketClose) {
//             arguments.push(self.parse_expression());
//
//             if matches!(self.tokens[self.position], Tokens::Comma) {
//                 self.advance();
//             }
//         }
//
//         self.advance(); // consume ']'
//
//         ASTNode::FunctionCall {
//             name: function_name,
//             arguments,
//         }
//     }
//     fn parse_print_statement(&mut self) -> ASTNode {
//         self.advance();
//
//         let target_node = self.parse_expression();
//
//         return ASTNode::PrintDecleration {
//             target: Box::new(target_node),
//         };
//     }
//     /// Entry point for parsing an expression.
//     pub fn parse_expression(&mut self) -> ASTNode {
//         let left_hand_side = self.parse_operand();
//
//         if self.is_at_end() {
//             return left_hand_side;
//         }
//
//         // if self.is_current_token_an_operator() {
//         //     return self.parse_binary_operation(left_hand_side);
//         // }
//
//         left_hand_side
//     }
//
//     /// Helper: Determines if the current token is a math operator.
//     fn is_current_token_an_operator(&self) -> bool {
//         matches!(
//             self.tokens[self.position],
//             Tokens::OperatorAdd
//                 | Tokens::OperatorSubtract
//                 | Tokens::OperatorMultiply
//                 | Tokens::OperatorDivide
//         )
//     }
//
//     /// Helper: Handles the left-side of an expression (Literals or Variables).
//     fn parse_operand(&mut self) -> ASTNode {
//         match &self.tokens[self.position] {
//             Tokens::LiteralInt(val) => {
//                 let node = ASTNode::LiteralInt { value: *val };
//                 self.advance();
//                 node
//             }
//             Tokens::Identifier(name) => {
//                 if self.position + 1 < self.tokens.len()
//                     && matches!(self.tokens[self.position + 1], Tokens::SquareBracketOpen)
//                 {
//                     return self.parse_function_call();
//                 }
//
//                 self.validate_variable_existence(name);
//
//                 let node = ASTNode::Identifier { name: name.clone() };
//
//                 self.advance();
//
//                 node
//             }
//             Tokens::OptionFalse => {
//                 self.advance();
//                 ASTNode::LiteralBool { value: false }
//             }
//             Tokens::OptionTrue => {
//                 self.advance();
//                 ASTNode::LiteralBool { value: true }
//             }
//             _ => panic!("Expected numeric literal or variable identifier"),
//         }
//     }
//
//     /// Helper: Constructs a BinaryOperation node recursively.
//     // fn parse_binary_operation(&mut self, left: ASTNode) -> ASTNode {
//     //     let left_type = self.get_node_type(&left);
//     //
//     //     if left_type == DataType::VarType::Bool {
//     //         panic!("Type Error: Cannot use boolean variable n arithmetic expression");
//     //     }
//     //
//     //     let operator_token = self.tokens[self.position].clone();
//     //     self.advance();
//     //
//     //     let right_hand_side = self.parse_expression();
//     //     let right_type = self.get_node_type(&right_hand_side);
//     //
//     //     if right_type == DataType::VarType::Bool {
//     //         panic!("Type Error: Cannot use Boolean Variable in artihmetic expression")
//     //     }
//     //
//     //     ASTNode::BinaryOperation {
//     //         left: Box::new(left),
//     //         operator: self.map_token_to_string(operator_token),
//     //         right: Box::new(right_hand_side),
//     //     }
//     // }
//
//     fn parse_assigment(&mut self) -> ASTNode {
//         let name = match &self.tokens[self.position] {
//             Tokens::Identifier(name) => name.clone(),
//             _ => panic!("Expected identifier"),
//         };
//
//         self.advance();
//
//         //consume the '='
//         self.advance();
//         let value = self.parse_expression();
//
//         ASTNode::Assingment {
//             name,
//             value: Box::new(value),
//         }
//     }
//
//     fn parse_statement(&mut self) -> ASTNode {
//         match &self.tokens[self.position] {
//             Tokens::VAR => self.parse_variable_declaration(),
//             Tokens::PRINT => self.parse_print_statement(),
//             Tokens::Identifier(_) => {
//                 if self.position + 1 < self.tokens.len()
//                     && self.tokens[self.position + 1] == Tokens::OperatorAssign
//                 {
//                     self.parse_assigment()
//                 } else {
//                     self.parse_expression()
//                 }
//             }
//             Tokens::LiteralInt(_) => self.parse_expression(),
//             Tokens::OptionTrue | Tokens::OptionFalse => self.parse_expression(),
//             _ => panic!(
//                 "Unexpected token in statement: {:?}",
//                 self.tokens[self.position]
//             ),
//         }
//     }
//
//     /// Helper: Validates if a variable exists in the registry.
//     fn validate_variable_existence(&self, name: &str) {
//         if !self
//             .variable_table
//             .check_variable_reference(&self.tokens[self.position])
//         {
//             panic!(
//                 "Semantic Error: Variable '{}' is referenced before declaration.",
//                 name
//             );
//         }
//     }
//
//     /// Helper: Converts token enum to a string representation.
//     fn map_token_to_string(&self, token: Tokens) -> String {
//         match token {
//             Tokens::OperatorAdd => "+".to_string(),
//             Tokens::OperatorSubtract => "-".to_string(),
//             Tokens::OperatorMultiply => "*".to_string(),
//             Tokens::OperatorDivide => "/".to_string(),
//             _ => unreachable!(),
//         }
//     }
//
//     fn is_at_end(&self) -> bool {
//         self.position >= self.tokens.len()
//     }
//     fn validate_bool_assignment(&self, value_node: &ASTNode) {
//         match value_node {
//             ASTNode::LiteralBool { .. } => (),
//
//             ASTNode::Identifier { name } => {
//                 if self.variable_table.get_type_by_name(name) != Some(DataType::VarType::Bool) {
//                     panic!("Type Error: Cannot assign non-boolean variable to boolean type.");
//                 }
//             }
//
//             _ => panic!("Constraint Error: Boolean variables must be assigned 'true' or 'false'."),
//         }
//     }
//
//     fn get_node_type(&self, node: &ASTNode) -> DataType::VarType {
//         match node {
//             ASTNode::LiteralInt { .. } => DataType::VarType::Int,
//             ASTNode::LiteralBool { .. } => DataType::VarType::Bool,
//             ASTNode::Identifier { name } => {
//                 // Use .expect() to turn the Option into the value, or crash with a helpful message
//                 self.variable_table
//                     .get_type_by_name(name)
//                     .expect(&format!("Semantic Error: Variable '{}' not found.", name))
//             }
//             ASTNode::BinaryOperaion { left, .. } => self.get_node_type(left),
//             _ => panic!("Unknown node type"),
//         }
//     }
// }
