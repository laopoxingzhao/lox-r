// 表达式相关定义，包含所有 AST 表达式节点类型
use crate::token::{LiteralType, Token};
use std::hash::Hash;

/// AST 表达式枚举，代表所有可能的表达式类型
#[derive(Debug, Clone)]
pub enum Expr {
    /// 赋值表达式
    Assignment(Assignment),
    /// 二元运算表达式
    Binary(Binary),
    /// 分组表达式 (括号)
    Grouping(Grouping),
    /// 字面量表达式
    Literal(Literal),
    /// 逻辑运算表达式 (and/or)
    Logical(Logical),
    /// 一元运算表达式
    Unary(Unary),
    /// 变量表达式
    Variable(Variable),
}
impl Expr {
    pub fn visit(){

    }
    
}

/// 赋值表达式结构体
#[derive(Debug, Clone)]
pub struct Assignment {
    /// 唯一 id
    pub uuid: usize,
    /// 变量名
    pub name: Token,
    /// 赋值的表达式
    pub value: Box<Expr>,
}

/// 二元运算表达式结构体
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// 分组表达式结构体 (括号)
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

/// 字面量表达式结构体
#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralType,
}

/// 逻辑运算表达式结构体 (and/or)
#[derive(Debug, Clone)]
pub struct Logical {
    pub uuid: usize,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// 一元运算表达式结构体
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

/// 变量表达式结构体
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}


/// 访问者模式 trait，用于遍历和处理不同类型的表达式节点
pub trait Visitor<T> {
    fn visit_assignment(&mut self, expr: &Assignment) -> T;
    fn visit_binary(&mut self, expr: &Binary) -> T;
    fn visit_grouping(&mut self, expr: &Grouping) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_logical(&mut self, expr: &Logical) -> T;
    fn visit_unary(&mut self, expr: &Unary) -> T;
    fn visit_variable(&mut self, expr: &Variable) -> T;
    fn visit_call(&mut self, expr: &Call) -> T;
    fn visit_get(&mut self, expr: &Get) -> T;
    fn visit_set(&mut self, expr: &Set) -> T;
    fn visit_this(&mut self, expr: &This) -> T;
    fn visit_super(&mut self, expr: &Super) -> T;
}

impl Expr {
    /// 访问者模式入口，根据表达式类型分派到对应的 visit 方法
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Assignment(assignment) => visitor.visit_assignment(assignment),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Logical(logical) => visitor.visit_logical(logical),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Variable(variable) => visitor.visit_variable(variable),
        }
    }

}