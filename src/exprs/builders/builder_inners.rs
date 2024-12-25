//! Defines the [BuilderInner] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-25

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use crate::exprs::{Expr,FmtExpr};
use crate::exprs::builders::Builder;
use crate::exprs::expr_inners::ExprInner;
use BuilderInner::*;

/// Representation of a builder of [Exprs][Expr]
pub(crate) enum BuilderInner<Head, Alloc>
  where Alloc: Allocator {
  /// Hole to be filled with an [Expr].
  BHole,
  /// Finished [Expr].
  BExpr(Expr<Head,Alloc>),
  /// [Expr] under construction.
  BPart(ExprInner<Head,Vec<Builder<Head,Alloc>,Alloc>,FmtExpr<Head,Alloc>>),
}

impl<Head, Alloc> Clone for BuilderInner<Head, Alloc>
  where Head: Clone, Alloc: Allocator + Clone {
  fn clone(&self) -> Self {
    match self {
      BHole          => BHole,
      BExpr(expr)    => BExpr(expr.clone()),
      BPart(builder) => BPart(builder.clone()),
    }
  }
  fn clone_from(&mut self, source: &Self) {
    match (source,self) {
      (BExpr(expr),   BExpr(dest)) => dest.clone_from(expr),
      (BPart(builder),BPart(dest)) => dest.clone_from(builder),
      (source,        dest)        => *dest = source.clone(),
    }
  }
}

impl<Head1, Head2, Alloc1, Alloc2> PartialEq<BuilderInner<Head2,Alloc2>> for BuilderInner<Head1, Alloc1>
  where Head1: PartialEq<Head2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &BuilderInner<Head2,Alloc2>) -> bool {
    match (self,rhs) {
      (BHole, _) | (_, BHole) => false,
      (BExpr(lhs),     BExpr(rhs))     => lhs == rhs,
      (BExpr(expr),    BPart(builder)) => expr == builder,
      (BPart(builder), BExpr(expr))    => builder == expr,
      (BPart(lhs),     BPart(rhs))     => lhs == rhs,
    }
  }
}
