use crate::ast::{ASTArena, ASTNode};
use crate::token::Token;

pub struct CodeGenerator<'a> {
    arena: &'a ASTArena,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(arena: &'a ASTArena) -> Self {
        Self { arena }
    }

    pub fn generate_program(&self, root_ids: &[usize]) -> String {
        let mut declarations_c = String::new();
        let mut main_body_c = String::new();

        for &id in root_ids {
            let node = self.arena.get(id);
            if let ASTNode::FunctionDeclaration { .. } = node {
                declarations_c.push_str(&self.generate_node(id));
                declarations_c.push_str("\n");
            } else {
                main_body_c.push_str(&format!("    {}", self.generate_node(id)));
            }
        }

        let mut final_code = String::new();
        final_code.push_str("#include <stdio.h>\n");
        final_code.push_str("#include <stdint.h>\n");
        final_code.push_str("#include <stdbool.h>\n\n");

        final_code.push_str(&declarations_c);

        final_code.push_str("int main() {\n");
        final_code.push_str(&main_body_c);
        final_code.push_str("    return 0;\n}\n");

        final_code
    }

    pub fn generate_node(&self, id: usize) -> String {
        let node = self.arena.get(id);

        match node {
            ASTNode::IntLiteral(val) => {
                format!("{}", val)
            }

            ASTNode::FloatLiteral(val) => {
                format!("{}", val)
            }

            ASTNode::BoolLiteral(val) => {
                if *val {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }

            ASTNode::StringLiteral(val) => {
                format!("\"{}\"", val)
            }

            ASTNode::Identifier(name) => name.clone(),

            ASTNode::BinaryOp {
                operator,
                left,
                right,
            } => {
                let left_c = self.generate_node(*left);
                let right_c = self.generate_node(*right);

                let op_str = match operator {
                    Token::Plus => "+",
                    Token::Minus => "-",
                    Token::Star => "*",
                    Token::Slash => "/",
                    _ => panic!("Codegen Error: Unhandled binary operator"),
                };

                format!("{} {} {}", left_c, op_str, right_c)
            }

            ASTNode::VariableDecleration {
                name,
                is_mutable,
                initializer,
            } => {
                let init_c = self.generate_node(*initializer);
                let const_prefix = if !*is_mutable { "const " } else { "" };
                format!("{}int64_t {} = {};\n", const_prefix, name, init_c)
            }

            ASTNode::SparkFeedPipeline {
                input,
                function_name,
            } => {
                let input_c = self.generate_node(*input);
                format!("{}({})", function_name, input_c)
            }

            ASTNode::PrintStatement { value } => {
                let inner_c = self.generate_node(*value);
                format!("printf(\"%lld\\n\", {});\n", inner_c)
            }

            ASTNode::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
            } => {
                let mut params_c = String::new();
                for (i, (p_name, p_type)) in params.iter().enumerate() {
                    if i > 0 {
                        params_c.push_str(", ");
                    }
                    let c_type = if p_type == "int" { "int64_t" } else { "double" };
                    params_c.push_str(&format!("{} {}", c_type, p_name));
                }

                let mut body_c = String::new();
                for &stmt_id in body {
                    body_c.push_str(&format!("    {}", self.generate_node(stmt_id)));
                }

                let c_ret_type = if return_type == "int" {
                    "int64_t"
                } else {
                    "void"
                };
                format!("{} {}({}) {{\n{}}}\n", c_ret_type, name, params_c, body_c)
            }

            ASTNode::ReturnStatement { value } => {
                if let Some(&val_idx) = value.as_ref() {
                    format!("return {};\n", self.generate_node(val_idx))
                } else {
                    format!("return;\n")
                }
            }
        }
    }
}
