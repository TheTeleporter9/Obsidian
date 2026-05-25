
use crate::token::Token;

#[derive(Debug,Clone,PartialEq)]
pub enum ASTNode {
    BinaryOp {
    operator: Token,
    left: usize,
    right: usize,
    },

    VariableDecleration {
    name: String,
    is_mutable: bool,
    initializer: usize,
    },

    SparkFeedPipeline {
    input: usize,
    function_name: String,
    },

   IntLiteral(i64),
   FloatLiteral(f64),
   BoolLiteral(bool),
   StringLiteral(String),
   Identifier(String),
}

#[derive(Debug, Clone)]
pub struct ASTArena {
    pool: Vec<ASTNode>,
}

impl ASTArena {
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
        }
    }

    pub fn alloc(&mut self, node: ASTNode) -> usize {
        let index = self.pool.len();
        self.pool.push(node);
        return index;
    }

    pub fn get(&self, index: usize) -> &ASTNode {
       return  &self.pool[index];
    }
}