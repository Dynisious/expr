//! Defines the [Lens] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-06

use alloc::alloc::Allocator;
use crate::exprs::{Expr,Token};
use crate::exprs::builders::Builder::{self,*};
use crate::exprs::builders::Node;

/// View into the [Expr] under construction by a [Builder].
pub struct Lens<'a,A>
  where A: Allocator {
  /// [Builder] being pointed at.
  pub builder: &'a mut Builder<A>
}

impl<'a,A> Lens<'a,A>
  where A: Allocator {
  /// Constructs a new Lens pointing at the root of the [Expr] under construction by `builder`.
  ///
  /// # Params
  ///
  /// builder --- The [Builder] to point into.  
  pub const fn from_builder(builder: &'a mut Builder<A>) -> Self { Self { builder } }
  /// Tests that `self` is pointing to a hole.
  pub const fn is_hole(&self) -> bool {
    match self.builder {
      BHole               => true,
      BExpr(_) | BNode(_) => false,
    }
  }
  /// Fills the hole with `builder`.
  ///
  /// Returns a Lens pointing at `builder`.
  ///
  /// # Params
  ///
  /// builder --- [Builder] to fill the hole with.  
  ///
  /// # Panics
  ///
  /// If this is not a hole; use [is_hole][Lens::is_hole] to check.
  pub fn fill_builder(&mut self, builder: Builder<A>) -> &mut Self {
    match self.builder {
      BHole => { *self.builder = builder; self },
      _     => panic!("attempted to fill a non-hole"),
    }
  }
  /// Fills the hole with `expr`.
  ///
  /// Returns a Lens pointing at `expr`.
  ///
  /// # Params
  ///
  /// expr --- [Expr] to fill the hole with.  
  ///
  /// # Panics
  ///
  /// If this is not a hole; use [is_hole][Lens::is_hole] to check.
  pub fn fill_expr(&mut self, expr: Expr<A>) -> &mut Self {
    self.fill_builder(Builder::from_expr(expr))
  }
  /// Fills the hole with an [Expr] built from `token`.
  ///
  /// Returns a Lens pointing at the new [Expr].
  ///
  /// # Params
  ///
  /// token --- [Token] which constitutes the [Expr].  
  /// alloc --- Allocator of the [Expr].  
  ///
  /// # Panics
  ///
  /// If this is not a hole; use [is_hole][Lens::is_hole] to check.
  pub fn fill_token(&mut self, token: Token<A>, alloc: A) -> &mut Self {
    self.fill_expr(Expr::from_token(token,alloc))
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// builder --- New child to push.  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_builder(&mut self, builder: Builder<A>) -> &mut Self {
    use core::mem;

    match &mut self.builder {
      lens@BExpr(_) => {
          let BExpr(expr) = mem::replace(*lens,Builder::hole())
            else { if cfg!(debug_assertions) { unreachable!("matched `self` as `BExpr`") }
                    else { unsafe { core::hint::unreachable_unchecked() } } };
          let mut node = Node::from_expr(expr);

          node.push_builder(builder);
          **lens = Builder::from_node(node);
          self
        },
      BHole       => panic!("attempted to add a child to a hole"),
      BNode(node) => { node.push_builder(builder); self },
    }
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// expr --- New child to push.  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_expr(&mut self, expr: Expr<A>) -> &mut Self {
    match self.builder {
      BExpr(lens) => { lens.push_expr(expr); self },
      _           => self.push_builder(Builder::from_expr(expr)),
    }
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// token --- [Token] to construct the new child from.  
  /// alloc --- Allocator of the [Expr].  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_token(&mut self, token: Token<A>, alloc: A) -> &mut Self {
    self.push_expr(Expr::from_token(token,alloc))
  }
  /// Returns a Lens pointing at the child at `child_index`.
  ///
  /// # Params
  ///
  /// child_index --- Index of the child node to index.  
  ///
  /// # Panics
  ///
  /// * `self` is pointing to a hole.
  /// * `child_index` is not in bounds.
  pub fn visit_child(&'a mut self, child_index: usize) -> Self {
    use core::mem;

    let builder = match &mut self.builder {
        BHole         => panic!("attempted to visit a child of a hole"),
        BNode(node)   => &mut node.children[child_index],
        lens@BExpr(_) => {
            let BExpr(expr) = mem::replace(*lens,Builder::hole())
              else { if cfg!(debug_assertions) { unreachable!("matched `self` as `BExpr`") }
                     else { unsafe { core::hint::unreachable_unchecked() } } };
            **lens = Builder::from_node(Node::from_expr(expr));
            let BNode(node) = lens
              else { if cfg!(debug_assertions) { unreachable!("matched `self` as `BExpr`") }
                     else { unsafe { core::hint::unreachable_unchecked() } } };

            &mut node.children[child_index]
          },
      };

    Self::from_builder(builder)
  }
}
