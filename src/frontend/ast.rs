
#[derive(Debug)]
pub enum ExprNode{
    Integer(i32),
    Op(Box<ExprNode>, String, Box<ExprNode>),
    UnaryOp(String, Box<ExprNode>)
}