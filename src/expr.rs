use crate::token::*;
pub enum Expr{
    Binary(Binary),
    // Grouping(Grouping),
    // Literal(Literal),
    // Unary(Unary),

}





struct Binary{
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}