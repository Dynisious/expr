//! Defines the [Expr] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-18

use alloc::alloc::Allocator;
use crate::nodes::Node;
use crate::nodes::impls::Owned;
pub use crate::nodes::impls::fmt_expr;
pub use self::tokens::Token;

pub mod builders;
pub(crate) mod tokens;

/// Expression tree of [Tokens][Token].
pub struct Expr<Alloc>(pub(crate) Node<Owned<Alloc>>)
  where Alloc: Allocator;

mod tests {
  #![cfg(test)]
  use alloc::alloc::Global;
  use crate::exprs::Expr;
  use crate::exprs::builders::Builder;

  #[test]
  fn test_expr_eq() {
    let alloc = Global;

    let tok_expr_a = Expr::from_str("a",alloc);
    let tok_expr_b = Expr::from_str("b",alloc);
    assert_eq!(tok_expr_a,tok_expr_a,"`Expr` of a single `Token` is not reflexive");

    let mut builder = Builder::from_str("a",alloc);
    builder.lens().push_str("a",alloc);
    let single_child_expr_a = builder.finish();
    assert_eq!(single_child_expr_a,single_child_expr_a,"`Expr` with a single child is not reflexive");

    assert_ne!(tok_expr_a,single_child_expr_a,"`Expr`s with different number of children match");

    assert_ne!(tok_expr_a,tok_expr_b,"`Expr`s with different head tokens match");

    let mut builder = Builder::from_str("a",alloc);
    builder.lens().push_str("b",alloc);
    let single_child_expr_b = builder.finish();
    assert_ne!(single_child_expr_a,single_child_expr_b,"`Expr`s with different child tokens match");
  }
}
