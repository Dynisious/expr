//! Defines the [Builder] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-18

use alloc::alloc::Allocator;
use core::fmt::{self,Debug,Formatter};
#[cfg(doc)]
use crate::exprs::Expr;
use crate::nodes::impls as expr_impls;
use crate::nodes::Node;
pub use self::lenses::Lens;
use BuilderInner::*;

mod lenses;
pub(crate) mod impls;

/// Internals of an expression builder.
pub(crate) enum BuilderInner<Impl>
  where Impl: expr_impls::Impl {
  /// A hole to be filled by an [Expr][expr_impls::Impl::Expr].
  BHole,
  /// A complete [Expr][expr_impls::Impl::Expr].
  BExpr(Impl::Expr),
  /// A partially built [Expr][expr_impls::Impl::Expr].
  BNode(Node<Impl::Builder>),
}

/// Builder of [Exprs][Expr].
///
/// Represents an [Expr] with holes to be filled.
pub struct Builder<Alloc>(pub(crate) BuilderInner<impls::Owned<Alloc>>)
  where Alloc: Allocator;

impl<Impl> BuilderInner<Impl>
  where Impl: expr_impls::Impl {
  /// Constructs a Builder from a [Expr][expr_impls::Impl::Expr].
  ///
  /// # Params
  ///
  /// expr --- Constituting [Expr][expr_impls::Impl::Expr].  
  pub const fn from_expr(expr: Impl::Expr) -> Self { BExpr(expr) }
  /// Constructs a Builder from a [Node].
  ///
  /// # Params
  ///
  /// node --- [Node] which constitutes the [Expr][expr_impls::Impl::Expr].  
  pub const fn from_node(node: Node<Impl::Builder>) -> Self { BNode(node) }
  /// Tests that there are no holes in the [Expr][expr_impls::Impl::Expr].
  pub const fn can_finish(&self) -> bool
    where Impl: ~const expr_impls::Impl {
    match self {
      BHole        => false,
      BExpr(_expr) => true,
      BNode(node)  => Impl::can_finish(node),
    }
  }
  /// Returns the built [Expr][expr_impls::Impl::Expr].
  ///
  /// # Panics
  ///
  /// If the [Expr][expr_impls::Impl::Expr] has unfilled holes; use [can_finish][Self::can_finish]
  /// to test if this function would panic.
  pub fn finish(self) -> Impl::Expr {
    debug_assert!(self.can_finish(),"called `finish` on a `Builder` with holes");

    match self {
      BHole       => panic!("called `finish` on a hole"),
      BExpr(expr) => expr,
      BNode(node) => Impl::finish(node),
    }
  }
}

impl<Impl> Debug for BuilderInner<Impl>
  where Impl: expr_impls::Impl, Impl::Expr: Debug, Node<Impl::Builder>: Debug {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    match self {
      BHole       => fmt.write_str("BHole"),
      BExpr(expr) => fmt.debug_tuple("BExpr").field(expr).finish(),
      BNode(node) => fmt.debug_tuple("BNode").field(node).finish(),
    }
  }
}
