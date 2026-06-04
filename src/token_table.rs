use crate::{
    AST::{self, ASTNode},
    DataType,
    tokens::Tokens,
};

pub struct variable_table {
    scopes: Vec<Vec<ASTNode>>,
}

impl variable_table {
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            panic!("Cannot pop global scope")
        }
    }

    pub fn add_variable_reference(&mut self, variable_data: ASTNode) {
        //Make shure that variable_data is of type Vaiable Decleration!
        if !matches!(
            variable_data,
            ASTNode::VariableDecleration {
                name: _,
                value: _,
                var_type: _
            }
        ) {
            panic!("Error at token_table! Parsed value is not of type VariableDecleration!");
        }
        println!("t_t-v_t: New variable added: {:?}", &variable_data);
        self.scopes.last_mut().unwrap().push(variable_data);
    }

    pub fn check_variable_reference(&self, checking_token: &Tokens) -> bool {
        for scope in self.scopes.iter().rev() {
            for variable in scope {
                println!("t_t-v_t: check variable reference {:?}", variable);
                if variable_table::var_name_match(checking_token, variable) {
                    println!("t_t-v_t: variable {:?} found and exist!", checking_token);
                    return true; // Found it!
                }
            }
        }

        false // Not found
    }

    pub fn var_name_match(token: &Tokens, node: &ASTNode) -> bool {
        match (token, node) {
            (Tokens::Identifier(token_name), ASTNode::VariableDecleration { name, .. }) => {
                token_name == name
            }

            (Tokens::Identifier(token_name), ASTNode::Identifier { name }) => token_name == name,
            _ => false,
        }
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<DataType::VarType> {
        for scope in self.scopes.iter().rev() {
            for variable in scope {
                if let ASTNode::VariableDecleration {
                    name: var_name,
                    var_type,
                    ..
                } = variable
                {
                    if var_name == name {
                        return Some(*var_type);
                    }
                }
            }
        }

        None
    }
}
pub struct FunctionTable {
    functions: Vec<ASTNode>,
}

impl FunctionTable {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn add_function_reference(&mut self, function_data: ASTNode) {
        // Make sure that function_data is of type FunctionDeclaration!
        if !matches!(
            function_data,
            ASTNode::FunctionDecleration {
                name: _,
                return_type: _,
                parameters: _,
                body: _,
                return_value: _,
            }
        ) {
            panic!("Error at function_table! Parsed value is not of type FunctionDecleration!");
        }

        println!("t_t-f_t: New function added: {:?}", &function_data);
        self.functions.push(function_data);
    }

    pub fn check_function_reference(&self, checking_token: &Tokens) -> bool {
        for function in &self.functions {
            println!("t_t-f_t: check function reference {:?}", function);

            if Self::func_name_match(checking_token, function) {
                println!("t_t-f_t: function {:?} found and exists!", checking_token);
                return true;
            }
        }

        false
    }

    pub fn func_name_match(token: &Tokens, node: &ASTNode) -> bool {
        match (token, node) {
            (Tokens::Identifier(token_name), ASTNode::FunctionDecleration { name, .. }) => {
                token_name == name
            }

            _ => false,
        }
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<DataType::VarType> {
        self.functions.iter().find_map(|function| match function {
            ASTNode::FunctionDecleration {
                name: func_name,
                return_type,
                ..
            } if func_name == name => Some(*return_type),

            _ => None,
        })
    }
}
