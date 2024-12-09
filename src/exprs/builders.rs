//! Defines the [Builder] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-06

use alloc::alloc::Allocator;
use crate::exprs::{Expr,Token};
pub use self::lenses::Lens;
use self::nodes::Node;
use Builder::*;

mod lenses;
mod nodes;

/// Builder of [Expr]s.
///
/// Represents an [Expr] with holes to be filled.
#[derive(Clone,Debug)]
pub enum Builder<A>
  where A: Allocator {
  /// A hole to be filled by an [Expr].
  BHole,
  /// A complete [Expr].
  BExpr(Expr<A>),
  /// A partially built [Expr].
  BNode(Node<A>),
}

impl<A> Builder<A>
  where A: Allocator {
  /// Construct a Builder which is a hole to be filled.
  pub const fn hole() -> Self { BHole }
  /// Constructs a Builder from an [Expr].
  ///
  /// # Params
  ///
  /// expr --- Constituting [Expr].  
  pub const fn from_expr(expr: Expr<A>) -> Self { BExpr(expr) }
  /// Constructs a Builder from a [Token].
  ///
  /// # Params
  ///
  /// token --- [Token] which constitutes the [Expr].  
  /// alloc --- Allocator of the [Expr].  
  pub const fn from_token(token: Token<A>, alloc: A) -> Self {
    Self::from_expr(Expr::from_token(token,alloc))
  }
  /// Constructs a Builder from a [Node].
  ///
  /// # Params
  ///
  /// node --- [Node] which constitutes the [Expr].  
  const fn from_node(node: Node<A>) -> Self { BNode(node) }
  /// Tests that there are no holes in the [Expr].
  ///
  /// # Examples
  ///
  /// ```
  /// # #![feature(allocator_api)]
  /// #
  /// # extern crate alloc;
  /// #
  /// # use alloc::alloc::Global;
  /// # use expr::exprs::Token;
  /// # use expr::exprs::builders::Builder;
  /// #
  /// # let alloc = Global;
  /// # let token_a = Token::from_str("a",alloc);
  /// let mut builder = Builder::hole();
  ///
  /// assert!(!builder.can_finish());
  /// 
  /// builder.lens().fill_token(token_a.clone(),alloc);
  /// assert!(builder.can_finish());
  ///
  /// builder.lens().push_token(token_a.clone(),alloc);
  /// assert!(builder.can_finish());
  ///
  /// builder.lens().push_builder(Builder::hole());
  /// assert!(!builder.can_finish());
  /// ```
  pub const fn can_finish(&self) -> bool {
    match self {
      BHole       => false,
      BExpr(_)    => true,
      BNode(node) => node.can_finish(),
    }
  }
  /// Returns the built [Expr].
  ///
  /// # Examples
  ///
  /// ```
  /// # #![feature(allocator_api)]
  /// #
  /// # extern crate alloc;
  /// #
  /// # use alloc::alloc::Global;
  /// # use expr::exprs::Token;
  /// # use expr::exprs::builders::Builder;
  /// #
  /// # let alloc = Global;
  /// # let token_a = Token::from_str("a",alloc);
  /// # let token_b = Token::from_str("b",alloc);
  /// let mut builder = Builder::from_token(token_a.clone(),alloc);
  ///
  /// assert_eq!("a",format!("{}",builder.clone().finish()));
  ///
  /// builder.lens().push_token(token_a.clone(),alloc);
  /// assert_eq!("a [a]",format!("{}",builder.clone().finish()));
  ///
  /// let mut lens = builder.lens();
  /// lens.push_builder(Builder::hole());
  /// lens.visit_child(1).fill_token(token_b.clone(),alloc);
  /// assert_eq!("a [a, b]",format!("{}",builder.clone().finish()));
  /// ```
  ///
  /// # Panics
  ///
  /// If the [Expr] has unfilled holes; use [can_finish][Builder::can_finish] to test if this
  /// function would panic.
  pub fn finish(self) -> Expr<A> {
    debug_assert!(self.can_finish(),"called `finish` on a `Builder` with holes");

    match self {
      BHole       => panic!("called `finish` on a hole"),
      BExpr(expr) => expr,
      BNode(node) => node.finish(),
    }
  }
  /// Returns a [Lens] at the root of the [Expr] under construction.
  pub const fn lens(&mut self) -> Lens<A> { Lens::from_builder(self) }
}
