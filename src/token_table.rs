use crate::{
    AST::{self, ASTNode},
    DataType,
    tokens::Tokens,
};

pub struct variable_table {
    index: i32,
    variables: Vec<ASTNode>,
}

impl variable_table {
    pub fn new() -> Self {
        Self {
            index: 0,
            variables: Vec::new(),
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
        self.variables.push(variable_data);
    }

    pub fn check_variable_reference(&self, checking_token: &Tokens) -> bool {
        for variable in &self.variables {
            println!("t_t-v_t: check variable reference {:?}", variable);
            if variable_table::var_name_match(checking_token, variable) {
                println!("t_t-v_t: variable {:?} found and exist!", checking_token);
                return true; // Found it!
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
        for variable in &self.variables {
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
        None
    }
}
