mod ast_gen;
mod constant;
mod location;
#[cfg(feature = "fold")]
mod fold_helpers;
mod impls;
#[cfg(feature = "unparse")]
mod unparse;

pub use ast_gen::*;

use serde::{Deserialize, Serialize};

/// Transforms a value prior to formatting it.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ConversionFlag {
    /// No conversion
    None = 0, // CPython uses -1 but not pleasure for us
    /// Converts by calling `str(<value>)`.
    Str = b's',
    /// Converts by calling `ascii(<value>)`.
    Ascii = b'a',
    /// Converts by calling `repr(<value>)`.
    Repr = b'r',
}

impl TryFrom<usize> for ConversionFlag {
    type Error = usize;
    fn try_from(b: usize) -> Result<Self, Self::Error> {
        let b = b.try_into().map_err(|_| b)?;
        match b {
            0 => Ok(Self::None),
            b's' => Ok(Self::Str),
            b'a' => Ok(Self::Ascii),
            b'r' => Ok(Self::Repr),
            b => Err(b as usize),
        }
    }
}


pub type Suite<U = ()> = Vec<Stmt<U>>;

pub enum DoMode {
    Async,
    Sync,
}

pub fn modify_rightmost_expr_of_statement(statement: &mut StmtKind, mut f: impl FnMut(&mut Expr) -> bool) -> bool {
    match statement {
        StmtKind::Expr { value } => { f(value) },
        StmtKind::Return { value } => {
            if let Some(value) = value {
                return f(value);
            }
            false
        },
        StmtKind::Assign { value, .. } => { f(value) },
        _ => false
    }
}

pub fn modify_rightmost_expr(expr: &mut Expr, mut f: impl FnMut(&mut Expr) -> bool) -> bool {
    match expr.node {
        ExprKind::BinOp { ref mut right , .. } => { modify_rightmost_expr(right, f) },
        ExprKind::BoolOp { ref mut values, .. } => {
            let len = values.len();
            modify_rightmost_expr(&mut values[len - 1], f)
        },
        ExprKind::UnaryOp { ref mut operand, .. } => { modify_rightmost_expr(operand, f) },
        _ => f(expr)
    }
}

// Applies the function f to the leftmost expression of a multi-expression. A multi-expression is any expression
// that has an expression on the left side of it. For example, in the expression `a + b + c`, `a` is the leftmost.
pub fn modify_leftmost_of_multi_expr(expr: &mut Expr, mut f: impl FnMut(&mut Expr) -> bool) -> bool {
    match expr.node {
        ExprKind::BoolOp { ref mut values, .. } => {
            let len = values.len();
            modify_leftmost_of_multi_expr(&mut values[len - 1], f)
        },
        ExprKind::NamedExpr { .. } => f(expr),
        ExprKind::BinOp { ref mut left , .. } => { modify_leftmost_of_multi_expr(left, f) },
        ExprKind::UnaryOp { .. } => f(expr),
        ExprKind::Lambda { .. } => f(expr),
        ExprKind::DoBlock { .. } => f(expr),
        ExprKind::EndOfBlockMarker { .. } => f(expr),
        ExprKind::IfExp { .. } => f(expr),
        ExprKind::Dict { .. } => f(expr),
        ExprKind::Set { .. } => f(expr),
        ExprKind::ListComp { .. } => f(expr),
        ExprKind::SetComp { .. } => f(expr),
        ExprKind::DictComp { .. } => f(expr),
        ExprKind::GeneratorExp { .. } => f(expr),
        ExprKind::Await { .. } => f(expr),
        ExprKind::Yield { .. } => f(expr),
        ExprKind::YieldFrom { .. } => f(expr),
        ExprKind::Compare { ref mut left, .. } => modify_leftmost_of_multi_expr(left, f),
        ExprKind::Call { ref mut func, .. } => modify_leftmost_of_multi_expr(func, f),
        ExprKind::FormattedValue { .. } => f(expr),
        ExprKind::JoinedStr { .. } => f(expr),
        ExprKind::Constant { .. } => f(expr),
        ExprKind::Attribute { ref mut value , .. } => modify_leftmost_of_multi_expr(value, f),
        ExprKind::Subscript{ ref mut value , .. } => modify_leftmost_of_multi_expr(value, f),
        ExprKind::Starred { .. } => f(expr),
        ExprKind::Name { .. } => f(expr),
        ExprKind::List { .. } => f(expr),
        ExprKind::Tuple { .. } => f(expr),
        ExprKind::Slice { .. } => f(expr),
    }
}