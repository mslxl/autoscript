
#[derive(Debug)]
pub enum ExprNode{
    Integer(i32),
    Float(f64),
    Op(Box<ExprNode>, String, Box<ExprNode>),
    UnaryOp(String, Box<ExprNode>)
}