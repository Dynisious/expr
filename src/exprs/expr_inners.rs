//! Defines the [ExprInner] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-25

use alloc::alloc::Allocator;
use alloc::vec::Vec;
#[cfg(doc)] use core::fmt::Display;
use core::fmt::{self,Debug,Formatter};
use core::{mem,ptr};
use crate::exprs::Expr;

/// Representation of an expression tree.
#[derive(Copy)]
pub(crate) struct ExprInner<Head, Children, Fmt> {
  /// Token at the head of this expression.
  pub head_token: Head,
  /// Child expressions of this expression.
  pub child_exprs: Children,
  /// Custom formatting method for [Display].
  pub fmt_expr: Fmt,
}

impl<Head, Children, Fmt> ExprInner<Head, Children, Fmt> {
  /// Constructs an ExprInner from parts.
  ///
  /// # Params
  ///
  /// head_token --- Token text at the head of this expression.  
  /// child_exprs --- Child expressions of this expression.  
  /// fmt_expr --- Custom formatting method for [Display].  
  pub const fn from_parts(head_token: Head, child_exprs: Children, fmt_expr: Fmt) -> Self {
    Self {head_token,child_exprs,fmt_expr}
  }
  /// Deconstructs an ExprInner into parts.
  ///
  /// Pre-inverse of [from_parts][Self::from_parts].
  pub const fn into_parts(self) -> (Head, Children, Fmt) {
    let head_token = unsafe { ptr::read(&self.head_token) };
    let child_exprs = unsafe { ptr::read(&self.child_exprs) };
    let fmt_expr = unsafe { ptr::read(&self.fmt_expr) };

    mem::forget(self);
    (head_token,child_exprs,fmt_expr)
  }
  /// References all fields.
  pub const fn fields(&self) -> (&Head, &Children, &Fmt) {
    (&self.head_token,&self.child_exprs,&self.fmt_expr)
  }
  /// References all fields.
  pub const fn fields_mut(&mut self) -> (&mut Head, &mut Children, &mut Fmt) {
    (&mut self.head_token,&mut self.child_exprs,&mut self.fmt_expr)
  }
  /// Formats the fields of `self`.
  pub fn fmt_fields(&self, fmt: &mut Formatter) -> fmt::Result
    where Head: Debug, Children: Debug, Fmt: Debug {
    write!(fmt,"head_token: {:?}, child_exprs: {:?}, fmt_expr: {:?}",
           self.head_token,self.child_exprs,self.fmt_expr)
  }
}

impl<Head, Children, Fmt> Clone for ExprInner<Head, Children, Fmt>
  where Head: Clone, Children: Clone, Fmt: Clone {
  fn clone(&self) -> Self {
    let head_token = self.head_token.clone();
    let child_exprs = self.child_exprs.clone();
    let fmt_expr = self.fmt_expr.clone();

    Self::from_parts(head_token,child_exprs,fmt_expr)
  }
  fn clone_from(&mut self, source: &Self) {
    self.head_token.clone_from(&source.head_token);
    self.child_exprs.clone_from(&source.child_exprs);
    self.fmt_expr.clone_from(&source.fmt_expr);
  }
}

impl<Head, Children, Fmt> Eq for ExprInner<Head, Children, Fmt>
  where Head: Eq, Children: Eq {}

impl<Head1, Head2, Children1, Children2, Fmt1, Fmt2> PartialEq<ExprInner<Head2, Children2, Fmt2>>
  for ExprInner<Head1, Children1, Fmt1>
  where Head1: PartialEq<Head2>, Children1: PartialEq<Children2> {
  fn eq(&self, rhs: &ExprInner<Head2, Children2, Fmt2>) -> bool {
    self.head_token == rhs.head_token && self.child_exprs == rhs.child_exprs
  }
}

impl<Head1, Alloc, Head2, Children, Fmt> PartialEq<Expr<Head2, Alloc>> for ExprInner<Head1, Children, Fmt>
  where Head1: PartialEq<Head2>, Alloc: Allocator, Children: PartialEq<Vec<Expr<Head2,Alloc>,Alloc>> {
  fn eq(&self, rhs: &Expr<Head2, Alloc>) -> bool { *self == rhs.0 }
}
