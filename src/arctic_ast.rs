

#[derive(Debug, PartialEq, Eq)]
pub enum ASTNode {
    Ident(String),
    Assign  {              
        ident: String,
        expr: Box<ASTNode>
    },
    Num(i64),
    BinaryOp {
        lhs: Box<ASTNode>,
        op: Operation,
        rlh: Box<ASTNode>,
    },
    Function {
        ident: String,
        arg: Box<ASTNode>
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulus,
}

