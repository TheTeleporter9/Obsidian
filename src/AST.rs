#[derive(Debug)]
pub enum ASTNode {
    VariableDecleration {
        name: String,
        value: Box<ASTNode>, //Makes shure that there can't be a memory overflow!
    },

    PrintDecleration {
        target: Box<ASTNode>,
    },

    BinaryOperaion {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },

    Identifier {
        name: String,
    },

    LiteralInt {
        value: i64,
    },
}
