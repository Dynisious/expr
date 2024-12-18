//! Defines the implementations of the [Node] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-17

#[cfg(doc)]
use core::fmt::Display;
#[cfg(doc)]
use crate::exprs::builders::BuilderInner;
use crate::nodes::Node;
pub(crate) use self::owned::Owned;
pub use self::owned::fmt_expr;

mod owned;

/// Interface of a [Node] implementation.
#[const_trait]
pub(crate) trait Impl {
  /// Text type of [Token][Self::Token].
  type Text;
  /// Formatting method for [Display].
  type Fmt;
  /// Token type for this implementation.
  type Token;
  /// Expression type for this implementation.
  type Expr;
  /// Container of child nodes.
  type Children;
  /// Implementation of a [Builder][BuilderInner] node.
  type Builder: Impl;

  /// Tests that there are no holes in the [Expr][Self::Expr].
  ///
  /// # Params
  ///
  /// node --- Partially built [Expr][Self::Expr] to test.  
  fn can_finish(node: &Node<Self::Builder>) -> bool;
  /// Returns the [Expr][Self::Expr] built by `node`.
  ///
  /// # Params
  ///
  /// node --- Partially built [Expr][Self::Expr] to finish.  
  fn finish(node: Node<Self::Builder>) -> Self::Expr;
}
