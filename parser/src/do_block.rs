use crate::ast;

pub enum StatementsOrDoBlock {
    Statements(ast::Suite),
    DoBlock(ast::Expr),
}