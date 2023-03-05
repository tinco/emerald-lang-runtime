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
