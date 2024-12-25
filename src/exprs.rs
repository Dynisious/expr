//! Defines the [Expr] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-25

use alloc::alloc::{Allocator,Global};
use alloc::vec::Vec;
use core::borrow::{Borrow,BorrowMut};
use core::fmt::{self,Debug,Display,Formatter};
use core::{mem,ptr};
use core::str::FromStr;
use crate::tokens::Token;
use self::builders::Builder;
use self::expr_inners::ExprInner;

pub mod builders;
mod expr_inners;

/// Formatting method for [Displaying][Display] [Exprs][Expr].
pub type FmtExpr<Head, Alloc> = fn(expr: &Expr<Head, Alloc>, fmt: &mut Formatter) -> fmt::Result;

/// The default `FmtExpr` implementation
pub fn fmt_expr<Head,Alloc>(expr: &Expr<Head, Alloc>, fmt: &mut Formatter) -> fmt::Result
  where Head: Display, Alloc: Allocator {
  write!(fmt,"{}",expr.head_token())?;

  let mut child_exprs = expr.child_exprs().iter();
  if let Some(child) = child_exprs.next() {
    write!(fmt," [{}",child)?;
    for child in child_exprs { write!(fmt,", {}",child)? }
    write!(fmt,"]")?;
  }

  Ok(())
}

/// Expression tree.
#[repr(transparent)]
pub struct Expr<Head, Alloc>(ExprInner<Head,Vec<Self,Alloc>,FmtExpr<Head,Alloc>>)
  where Alloc: Allocator;

impl<Head, Alloc> Expr<Head, Alloc>
  where Alloc: Allocator {
  /// Constructs an Expr from parts.
  ///
  /// # Params
  ///
  /// head_token --- Token at the head of this expression.  
  /// child_exprs --- Child expressions of this expression.  
  /// fmt_expr --- Custom formatting method for [Display].  
  pub const fn from_parts(head_token: Head, child_exprs: Vec<Self, Alloc>,
                          fmt_expr: FmtExpr<Head, Alloc>) -> Self {
    Self(ExprInner::from_parts(head_token,child_exprs,fmt_expr))
  }
  /// Deconstructs an Expr into parts.
  ///
  /// Pre-inverse of [from_parts][Self::from_parts].
  pub const fn into_parts(self) -> (Head, Vec<Self, Alloc>, FmtExpr<Head, Alloc>) {
    let inner = unsafe { ptr::read(&self.0) };

    mem::forget(self);
    inner.into_parts()
  }
  /// Constructs an Expr from a token.
  ///
  /// # Params
  ///
  /// head_token --- Token at the head of this expression.  
  /// allocator --- Allocator of the expression.  
  pub const fn from_token_in(head_token: Head, allocator: Alloc) -> Self
    where Head: Display {
    let child_exprs = Vec::new_in(allocator);
    let fmt_expr: FmtExpr<Head,Alloc> = fmt_expr;

    Self::from_parts(head_token,child_exprs,fmt_expr)
  }
  /// Constructs a default Expr.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the expression.  
  pub fn new_in(allocator: Alloc) -> Self
    where Head: Default + Display {
    let head_token = Default::default();

    Self::from_token_in(head_token,allocator)
  }
  /// References all fields.
  pub const fn fields(&self) -> (&Head, &Vec<Self, Alloc>, &FmtExpr<Head, Alloc>) {
    self.0.fields()
  }
  /// References all fields.
  pub const fn fields_mut(&mut self) -> (&mut Head, &mut Vec<Self, Alloc>, &mut FmtExpr<Head, Alloc>) {
    self.0.fields_mut()
  }
  /// Token at the head of this expression.
  pub const fn head_token(&self) -> &Head { &self.0.head_token }
  /// Token at the head of this expression.
  pub const fn head_token_mut(&mut self) -> &mut Head { &mut self.0.head_token }
  /// Token at the head of this expression.
  pub const fn child_exprs(&self) -> &Vec<Self, Alloc> { &self.0.child_exprs }
  /// Token at the head of this expression.
  pub const fn child_exprs_mut(&mut self) -> &mut Vec<Self, Alloc> { &mut self.0.child_exprs }
  /// Token at the head of this expression.
  pub const fn fmt_expr(&self) -> &FmtExpr<Head, Alloc> { &self.0.fmt_expr }
  /// Token at the head of this expression.
  pub const fn fmt_expr_mut(&mut self) -> &mut FmtExpr<Head, Alloc> { &mut self.0.fmt_expr }
  /// Formats the fields of `self`.
  pub fn fmt_fields(&self, fmt: &mut Formatter) -> fmt::Result
    where Head: Debug { self.0.fmt_fields(fmt) }
}

impl<Alloc> Expr<Token<Alloc>, Alloc>
  where Alloc: Allocator {
  /// Constructs an Expr from a [Token].
  ///
  /// # Params
  ///
  /// head_token --- [Token] at the head of this expression.  
  pub fn from_token(head_token: Token<Alloc>) -> Self
    where Alloc: Clone {
    let allocator = head_token.allocator().clone();

    Self::from_token_in(head_token,allocator)
  }
  /// Constructs an Expr from text.
  ///
  /// # Params
  ///
  /// head_token --- Token text at the head of this expression.  
  /// allocator --- Allocator of the expression.  
  pub fn from_str_in(head_token: &str, allocator: Alloc) -> Self
    where Alloc: Clone { Self::from_token(Token::from_str_in(head_token,allocator)) }
}

impl<Head, Alloc> Clone for Expr<Head, Alloc>
  where Head: Clone, Alloc: Allocator + Clone {
  fn clone(&self) -> Self { Self(self.0.clone()) }
  fn clone_from(&mut self, source: &Self) { self.0.clone_from(&source.0) }
}

impl<Alloc> From<Token<Alloc>> for Expr<Token<Alloc>, Alloc>
  where Alloc: Allocator + Clone {
  fn from(from: Token<Alloc>) -> Self { Self::from_token(from) }
}

impl From<&str> for Expr<Token<Global>, Global> {
  fn from(from: &str) -> Self { Self::from_str_in(from,Global) }
}

impl FromStr for Expr<Token<Global>, Global> {
  type Err = !;

  fn from_str(text: &str) -> Result<Self,Self::Err> { Ok(text.into()) }
}

impl<Head, Alloc> Eq for Expr<Head, Alloc>
  where Head: Eq, Alloc: Allocator {}

impl<Head1, Alloc, Head2, Children, Fmt> PartialEq<ExprInner<Head2, Children, Fmt>>
  for Expr<Head1, Alloc>
  where Head1: PartialEq<Head2>, Alloc: Allocator, Vec<Self,Alloc>: PartialEq<Children> {
  fn eq(&self, rhs: &ExprInner<Head2, Children, Fmt>) -> bool { self.0 == *rhs }
}

impl<Head1, Alloc1, Head2, Alloc2> PartialEq<Expr<Head2, Alloc2>> for Expr<Head1, Alloc1>
  where Head1: PartialEq<Head2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Expr<Head2, Alloc2>) -> bool { *self == rhs.0 }
}

impl<Head1, Head2, Alloc1, Alloc2> PartialEq<Builder<Head2,Alloc2>> for Expr<Head1, Alloc1>
  where Head1: PartialEq<Head2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Builder<Head2,Alloc2>) -> bool { self.eq_builder(rhs) }
}

impl<Head, Alloc> Borrow<ExprInner<Head, Vec<Self, Alloc>, FmtExpr<Head, Alloc>>>
  for Expr<Head, Alloc>
  where Alloc: Allocator {
  fn borrow(&self) -> &ExprInner<Head, Vec<Self, Alloc>, FmtExpr<Head, Alloc>> { &self.0 }
}

impl<Head, Alloc> BorrowMut<ExprInner<Head, Vec<Self, Alloc>, FmtExpr<Head, Alloc>>>
  for Expr<Head, Alloc>
  where Alloc: Allocator {
  fn borrow_mut(&mut self) -> &mut ExprInner<Head, Vec<Self, Alloc>, FmtExpr<Head, Alloc>> {
    &mut self.0
  }
}

impl<Head, Alloc> Display for Expr<Head, Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { (self.fmt_expr())(self,fmt) }
}

impl<Head, Alloc> Debug for Expr<Head, Alloc>
  where Head: Debug, Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    write!(fmt,"Expr {{ ")?;
    self.fmt_fields(fmt)?;
    write!(fmt," }}")
  }
}

mod tests {
  #![cfg(test)]
  use alloc::alloc::Global;
  use crate::exprs::Expr;
  use crate::exprs::builders::Builder;

  #[test]
  fn test_expr_eq() {
    let alloc = Global;

    let tok_expr_a = Expr::from_str_in("a",alloc);
    let tok_expr_b = Expr::from_str_in("b",alloc);
    assert_eq!(tok_expr_a,tok_expr_a,"`Expr` of a single `Token` is not reflexive");

    let mut builder = Builder::from_str_in("a",alloc);
    builder.push_str_in("a",alloc);
    let single_child_expr_a = builder.finish();
    assert_eq!(single_child_expr_a,single_child_expr_a,"`Expr` with a single child is not reflexive");

    assert_ne!(tok_expr_a,single_child_expr_a,"`Expr`s with different number of children match");

    assert_ne!(tok_expr_a,tok_expr_b,"`Expr`s with different head tokens match");

    let mut builder = Builder::from_str_in("a",alloc);
    builder.push_str_in("b",alloc);
    let single_child_expr_b = builder.finish();
    assert_ne!(single_child_expr_a,single_child_expr_b,"`Expr`s with different child tokens match");
  }
}
