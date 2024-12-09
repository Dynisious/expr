//! Defines the [Node] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-06

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::fmt::{self,Formatter};
use core::mem::ManuallyDrop;
use crate::exprs::{Expr,Token};
use crate::exprs::builders::Builder;

/// [Expr] with partially constructed children.
#[derive(Clone,Debug)]
pub struct Node<A>
  where A: Allocator {
  /// Symbol at the head of the [Expr].
  pub head_token: Token<A>,
  /// Child expressions.
  pub children: Vec<Builder<A>,A>,
  /// Formatting method of the [Expr].
  pub fmt: fn(&Expr<A>, &mut Formatter) -> fmt::Result,
}

impl<A> Node<A>
  where A: Allocator {
  /// Constructs a new Node from parts.
  ///
  /// # Params
  ///
  /// head_token --- Symbol at the head of the [Expr].  
  /// children --- Child expressions.  
  /// fmt --- Formatting method of the [Expr].  
  pub const fn from_parts(head_token: Token<A>, children: Vec<Builder<A>,A>,
                          fmt: fn(&Expr<A>, &mut Formatter) -> fmt::Result) -> Self {
    Self { head_token, children, fmt }
  }
  /// Constructs a new Node from an [Expr].
  ///
  /// # Params
  ///
  /// expr --- [Expr] to build upon.  
  pub fn from_expr(expr: Expr<A>) -> Self {
    let Expr { head_token, children, fmt } = expr;
    let children = map_in_place::vec::alloc::map(children,Builder::from_expr);

    Self::from_parts(head_token,children,fmt)
  }
  /// Tests that there are no holes in the [Expr].
  pub const fn can_finish(&self) -> bool {
    let mut index = 0;
    while index < self.children.len() {
      let child = &self.children.as_slice()[index];
      index += 1;

      if !child.can_finish() { return false }
    }

    true
  }
  /// Returns the built [Expr].
  ///
  /// # Panics
  ///
  /// If the [Expr] has unfilled holes; use [can_finish][Node::can_finish] to test if this
  /// function would panic.
  pub fn finish(self) -> Expr<A> {
    use core::ptr;

    let self_ = ManuallyDrop::new(self);
    let head_token = unsafe { ptr::read(&self_.head_token) };
    let children = map_in_place::vec::alloc::map(unsafe { ptr::read(&self_.children) },
                                                 Builder::finish);

    Expr::from_parts(head_token,children,self_.fmt)
  }
  /// Nests a [Builder] at the end of the `self`s children.
  ///
  /// # Params
  ///
  /// builder --- New [Builder] to append.  
  pub fn push_builder(&mut self, builder: Builder<A>) -> &mut Self {
    self.children.push(builder); self
  }
  /// Adds an [Expr] to the end of the `self`s children.
  ///
  /// # Params
  ///
  /// expr --- New child to append.  
  pub fn push_expr(&mut self, expr: Expr<A>) -> &mut Self {
    self.push_builder(Builder::from_expr(expr))
  }
}
