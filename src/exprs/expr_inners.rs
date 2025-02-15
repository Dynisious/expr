//! Defines the [ExprInner] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

use alloc::alloc::Allocator;
use alloc::vec::Vec;
#[cfg(doc)] use core::fmt::Display;
use core::fmt::{self,Debug,Formatter};
use core::{mem,ptr};
use crate::exprs::Expr;

/// Representation of an expression tree of `Token`.
#[derive(Clone,Copy)]
pub struct ExprInner<Token, Children, Fmt> {
  /// Token at the head of this expression.
  pub head_token: Token,
  /// Child expressions of this expression.
  pub child_exprs: Children,
  /// Custom formatting method for [Display].
  pub fmt_expr: Fmt,
}

impl<Token, Children, Fmt> ExprInner<Token, Children, Fmt> {
  /// Constructs an ExprInner from parts.
  ///
  /// # Params
  ///
  /// head_token --- Token text at the head of this expression.  
  /// child_exprs --- Child expressions of this expression.  
  /// fmt_expr --- Custom formatting method for [Display].  
  pub const fn from_parts(head_token: Token, child_exprs: Children, fmt_expr: Fmt) -> Self {
    Self {head_token,child_exprs,fmt_expr}
  }
  /// Deconstructs an ExprInner into parts.
  ///
  /// Inverse of [from_parts][Self::from_parts].
  pub const fn into_parts(self) -> (Token, Children, Fmt) {
    let head_token = unsafe { ptr::read(&self.head_token) };
    let child_exprs = unsafe { ptr::read(&self.child_exprs) };
    let fmt_expr = unsafe { ptr::read(&self.fmt_expr) };

    mem::forget(self);
    (head_token,child_exprs,fmt_expr)
  }
  /// Formats the fields of `self`.
  pub fn fmt_fields(&self, fmt: &mut Formatter) -> fmt::Result
    where Token: Debug, Children: Debug, Fmt: Debug {
    write!(fmt,"head_token: {:?}, child_exprs: {:?}, fmt_expr: {:?}",
           self.head_token,self.child_exprs,self.fmt_expr)
  }
}

impl<Token, Children, Fmt> Eq for ExprInner<Token, Children, Fmt>
  where Token: Eq, Children: Eq {}

impl<Token1, Token2, Children1, Children2, Fmt1, Fmt2> PartialEq<ExprInner<Token2, Children2, Fmt2>>
  for ExprInner<Token1, Children1, Fmt1>
  where Token1: PartialEq<Token2>, Children1: PartialEq<Children2> {
  fn eq(&self, rhs: &ExprInner<Token2, Children2, Fmt2>) -> bool {
    self.head_token == rhs.head_token && self.child_exprs == rhs.child_exprs
  }
}

impl<Token1, Alloc, Token2, Children, Fmt> PartialEq<Expr<Token2,Alloc>> for ExprInner<Token1, Children, Fmt>
  where Token1: PartialEq<Token2>, Alloc: Allocator, Children: PartialEq<Vec<Expr<Token2,Alloc>,Alloc>> {
  fn eq(&self, rhs: &Expr<Token2, Alloc>) -> bool { *self == rhs.inner }
}
