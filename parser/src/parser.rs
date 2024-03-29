//! Emerald parsing.
//!
//! Use this module to parse emerald code into an AST.
//! There are three ways to parse emerald code. You could
//! parse a whole program, a single statement, or a single
//! expression.

use crate::lexer::{LexResult, Tok};
pub use crate::mode::Mode;
use crate::{ast, error::ParseError, lexer, emerald};
use itertools::Itertools;
use std::iter;

/*
 * Parse emerald code.
 * Grammar may be inspired by antlr grammar for emerald:
 * https://github.com/antlr/grammars-v4/tree/master/emerald3
 */

/// Parse a full emerald program, containing usually multiple lines.
pub fn parse_program(source: &str, source_path: &str) -> Result<ast::Suite, ParseError> {
    parse(source, Mode::Module, source_path).map(|top| match top {
        ast::Mod::Module { body, .. } => body,
        _ => unreachable!(),
    })
}

/// Parses a emerald expression
///
/// # Example
/// ```
/// extern crate num_bigint;
/// use emerald_lang_parser::{parser, ast};
/// let expr = parser::parse_expression("1 + 2", "<embedded>").unwrap();
///
/// assert_eq!(
///     expr,
///     ast::Expr {
///         location: ast::Location::new(1, 0),
///         end_location: Some(ast::Location::new(1, 5)),
///         custom: (),
///         node: ast::ExprKind::BinOp {
///             left: Box::new(ast::Expr {
///                 location: ast::Location::new(1, 0),
///                 end_location: Some(ast::Location::new(1, 1)),
///                 custom: (),
///                 node: ast::ExprKind::Constant {
///                     value: ast::Constant::Int(1.into()),
///                     kind: None,
///                 }
///             }),
///             op: ast::Operator::Add,
///             right: Box::new(ast::Expr {
///                 location: ast::Location::new(1, 4),
///                 end_location: Some(ast::Location::new(1, 5)),
///                 custom: (),
///                 node: ast::ExprKind::Constant {
///                     value: ast::Constant::Int(2.into()),
///                     kind: None,
///                 }
///             })
///         }
///     },
/// );
///
/// ```
pub fn parse_expression(source: &str, path: &str) -> Result<ast::Expr, ParseError> {
    parse(source, Mode::Expression, path).map(|top| match top {
        ast::Mod::Expression { body } => *body,
        _ => unreachable!(),
    })
}

// Parse a given source code
pub fn parse(source: &str, mode: Mode, source_path: &str) -> Result<ast::Mod, ParseError> {
    let lxr = lexer::make_tokenizer(source);
    let marker_token = (Default::default(), mode.to_marker(), Default::default());
    let tokenizer = iter::once(Ok(marker_token))
        .chain(lxr)
        .filter_ok(|(_, tok, _)| !matches!(tok, Tok::Comment));

    emerald::TopParser::new()
        .parse(tokenizer)
        .map_err(|e| crate::error::parse_error_from_lalrpop(e, source_path))
}

// Parse a given token iterator.
pub fn parse_tokens(
    lxr: impl IntoIterator<Item = LexResult>,
    mode: Mode,
    source_path: &str,
) -> Result<ast::Mod, ParseError> {
    let marker_token = (Default::default(), mode.to_marker(), Default::default());
    let tokenizer = iter::once(Ok(marker_token))
        .chain(lxr)
        .filter_ok(|(_, tok, _)| !matches!(tok, Tok::Comment));

    emerald::TopParser::new()
        .parse(tokenizer)
        .map_err(|e| crate::error::parse_error_from_lalrpop(e, source_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let parse_ast = parse_program("", "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_string() {
        let source = String::from("'Hello world'");
        let parse_ast = parse_program(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_f_string() {
        let source = String::from("f'Hello world'");
        let parse_ast = parse_program(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_print_hello() {
        let source = String::from("print('Hello world')");
        let parse_ast = parse_program(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_print_2() {
        let source = String::from("print('Hello world', 2)");
        let parse_ast = parse_program(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_kwargs() {
        let source = String::from("my_func('positional', keyword=2)");
        let parse_ast = parse_program(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_if_elif_else() {
        let source = String::from("if 1: 10\nelif 2: 20\nelse: 30");
        let parse_ast = parse_program(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_lambda() {
        let source = "lambda x, y: x * y"; // lambda(x, y): x * y";
        let parse_ast = parse_program(source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_tuples() {
        let source = "a, b = 4, 5";

        insta::assert_debug_snapshot!(parse_program(source, "<test>").unwrap());
    }

    #[test]
    fn test_parse_class() {
        let source = "\
class Foo extends A:
 def initialize():
  pass
 def method_with_default(self, arg='default'):
  pass";
        insta::assert_debug_snapshot!(parse_program(source, "<test>").unwrap());
    }

    #[test]
    fn test_do_blocks() {
        let source = "\
some_fun() do:
  print('Hello world')
print('ok')";
        insta::assert_debug_snapshot!(parse_program(source, "<test>").unwrap());
    }

    #[test]
    fn test_do_blocks_inside_assignment() {
        let source = "\
a = some_fun() do:
  print('Hello world')
print('ok')";
        insta::assert_debug_snapshot!(parse_program(source, "<test>").unwrap());
    } 
 
// TODO there are a dozen situations where the do block should be associated with an expression
// but the last expression is part of a statement. Maybe we could keep track of the last expression
// somehow.

//     #[test]
//     fn test_do_blocks_inside_expression() {
//         let source = "\
// 5 + some_fun() do:
//   print('Hello world')
// print('ok')";
//         insta::assert_debug_snapshot!(parse_program(source, "<test>").unwrap());
//     } 

    #[test]
    fn test_parse_dict_comprehension() {
        let source = String::from("{x1: x2 for y in z}");
        let parse_ast = parse_expression(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_list_comprehension() {
        let source = String::from("[x for y in z]");
        let parse_ast = parse_expression(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_double_list_comprehension() {
        let source = String::from("[x for y, y2 in z for a in b if a < 5 if a > 10]");
        let parse_ast = parse_expression(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_generator_comprehension() {
        let source = String::from("(x for y in z)");
        let parse_ast = parse_expression(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_named_expression_generator_comprehension() {
        let source = String::from("(x := y + 1 for y in z)");
        let parse_ast = parse_expression(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_parse_if_else_generator_comprehension() {
        let source = String::from("(x if y else y for y in z)");
        let parse_ast = parse_expression(&source, "<test>").unwrap();
        insta::assert_debug_snapshot!(parse_ast);
    }
}
