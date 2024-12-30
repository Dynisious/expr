//! Defines the [Expr] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-30

use alloc::alloc::{Allocator,Global};
use alloc::vec::Vec;
use core::fmt::{self,Debug,Display,Formatter};
use core::{mem,ptr};
use core::str::FromStr;
use core::ops::{Deref,DerefMut};
use crate::tokens::Token;
pub use self::builders::Builder;
pub use self::expr_inners::ExprInner;

mod builders;
mod expr_inners;

/// Formatting method for [Displaying][Display] [Exprs][Expr].
pub type FmtExpr<Token, Alloc> = fn(expr: &Expr<Token, Alloc>, fmt: &mut Formatter) -> fmt::Result;

/// The default `FmtExpr` implementation.
pub fn fmt_expr<Token,Alloc>(expr: &Expr<Token, Alloc>, fmt: &mut Formatter) -> fmt::Result
  where Token: Display, Alloc: Allocator {
  write!(fmt,"{}",expr.head_token)?;

  let mut child_exprs = expr.child_exprs.iter();
  if let Some(child) = child_exprs.next() {
    write!(fmt," [{}",child)?;
    for child in child_exprs { write!(fmt,", {}",child)? }
    write!(fmt,"]")?;
  }

  Ok(())
}

/// Expression tree of `Token`s.
#[repr(transparent)]
pub struct Expr<Token, Alloc = Global>
  where Alloc: Allocator {
  /// The inner expression representation.
  pub inner: ExprInner<Token,Vec<Self,Alloc>,FmtExpr<Token,Alloc>>,
}

impl<Token, Alloc> Expr<Token, Alloc>
  where Alloc: Allocator {
  /// Constructs an Expr from parts.
  ///
  /// # Params
  ///
  /// head_token --- Token at the head of this expression.  
  /// child_exprs --- Child expressions of this expression.  
  /// fmt_expr --- Custom formatting method for [Display].  
  pub const fn from_parts(head_token: Token, child_exprs: Vec<Self, Alloc>,
                          fmt_expr: FmtExpr<Token, Alloc>) -> Self {
    let inner = ExprInner::from_parts(head_token,child_exprs,fmt_expr);

    Self{inner}
  }
  /// Deconstructs an Expr into parts.
  ///
  /// Inverse of [from_parts][Self::from_parts].
  pub const fn into_parts(self) -> (Token, Vec<Self, Alloc>, FmtExpr<Token, Alloc>) {
    let inner = unsafe { ptr::read(&self.inner) };

    mem::forget(self);
    inner.into_parts()
  }
  /// Constructs an Expr from a token.
  ///
  /// # Params
  ///
  /// head_token --- Token at the head of this expression.  
  /// allocator --- Allocator of the expression.  
  pub const fn from_token_in(head_token: Token, allocator: Alloc) -> Self
    where Token: Display {
    let child_exprs = Vec::new_in(allocator);
    let fmt_expr: FmtExpr<Token,Alloc> = fmt_expr;

    Self::from_parts(head_token,child_exprs,fmt_expr)
  }
  /// Constructs a default Expr.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the expression.  
  pub fn new_in(allocator: Alloc) -> Self
    where Token: Default + Display {
    let head_token = Default::default();

    Self::from_token_in(head_token,allocator)
  }
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

impl Expr<Token<Global>, Global> {
  /// Constructs an Expr from text.
  ///
  /// # Params
  ///
  /// head_token --- Token text at the head of this expression.  
  pub fn from_str(head_token: &str) -> Self { Self::from_str_in(head_token,Global) }
}

impl<Token, Alloc> Clone for Expr<Token, Alloc>
  where Token: Clone, Alloc: Allocator + Clone {
  fn clone(&self) -> Self {
    let inner = self.inner.clone();

    Self{inner}
  }
  fn clone_from(&mut self, source: &Self) { self.inner.clone_from(&source.inner) }
}

impl<Alloc> From<Token<Alloc>> for Expr<Token<Alloc>, Alloc>
  where Alloc: Allocator + Clone {
  fn from(from: Token<Alloc>) -> Self { Self::from_token(from) }
}

impl From<&str> for Expr<Token<Global>, Global> {
  fn from(from: &str) -> Self { Self::from_str(from) }
}

impl FromStr for Expr<Token<Global>, Global> {
  type Err = !;

  fn from_str(text: &str) -> Result<Self,Self::Err> { Ok(text.into()) }
}

impl<Token, Alloc> Eq for Expr<Token, Alloc>
  where Token: Eq, Alloc: Allocator {}

impl<Token1, Alloc, Token2, Children, Fmt> PartialEq<ExprInner<Token2, Children, Fmt>>
  for Expr<Token1, Alloc>
  where Token1: PartialEq<Token2>, Alloc: Allocator, Vec<Self,Alloc>: PartialEq<Children> {
  fn eq(&self, rhs: &ExprInner<Token2, Children, Fmt>) -> bool { self.inner == *rhs }
}

impl<Token1, Alloc1, Token2, Alloc2> PartialEq<Expr<Token2, Alloc2>> for Expr<Token1, Alloc1>
  where Token1: PartialEq<Token2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Expr<Token2, Alloc2>) -> bool { *self == rhs.inner }
}

impl<Token, Alloc> Deref for Expr<Token, Alloc>
  where Alloc: Allocator {
  type Target = ExprInner<Token,Vec<Self,Alloc>,FmtExpr<Token,Alloc>>;

  fn deref(&self) -> &Self::Target { &self.inner }
}

impl<Token, Alloc> DerefMut for Expr<Token, Alloc>
  where Alloc: Allocator {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<Token, Alloc> Display for Expr<Token, Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { (self.fmt_expr)(self,fmt) }
}

impl<Token, Alloc> Debug for Expr<Token, Alloc>
  where Token: Debug, Alloc: Allocator {
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
