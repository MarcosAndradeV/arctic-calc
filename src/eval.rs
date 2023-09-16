use crate::{arctic_ast::{ASTNode,Operation}, enviroment::Env};


pub struct Eval {
    env: Env
}

impl Eval {
    pub fn new(env: Env) -> Self {
        Self { env }
     }

    pub fn eval(&mut self, node: ASTNode) -> i64 {
        match node {
            ASTNode::Ident(i) => { 
                match self.env.get_value(i) {
                    Some(v) => *v,
                    None => 0,
                }
            },
            ASTNode::Num(n) => n,
            ASTNode::Assign { ident, expr } => {
                let e = self.eval(*expr);
                self.env.set_value(ident, e);
                e
            },
            ASTNode::Function { ident, arg } => {
                self.eval(*arg)
            },
            ASTNode::BinaryOp 
            { lhs, op, rlh } => {

                let lhs_val = self.eval(*lhs);
                let rlh_val = self.eval(*rlh);
                match op {
                    Operation::Add => lhs_val + rlh_val,
                    Operation::Subtract => lhs_val - rlh_val,
                    Operation::Multiply => lhs_val * rlh_val,
                    Operation::Divide => lhs_val / rlh_val,
                    Operation::Power => lhs_val.pow(rlh_val as u32)  ,
                    Operation::Modulus => lhs_val % rlh_val,
                }
            },
        }
    }
}