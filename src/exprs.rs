//! Defines the [Expr] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-06

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::fmt::{self,Display,Formatter};
pub use self::tokens::Token;

pub mod builders;
mod tokens;

/// The default formatting method for [Expr] values.
///
/// # Examples
///
/// ```
/// # #![feature(allocator_api)]
/// # extern crate alloc;
/// #
/// # use alloc::alloc::Global;
/// # use expr::exprs::{Expr,Token};
/// #
/// # let alloc = Global;
/// # let token_a = Token::from_str("a",alloc);
/// let expr_a = Expr::from_token(token_a,alloc);
///
/// assert_eq!("a",format!("{}",expr_a));
/// ```
///
/// ```
/// # #![feature(allocator_api)]
/// # extern crate alloc;
/// #
/// # use alloc::alloc::Global;
/// # use expr::exprs::{Expr,Token};
/// #
/// # let alloc = Global;
/// # let token_a = Token::from_str("a",alloc);
/// let mut expr_a = Expr::from_token(token_a.clone(),alloc);
/// expr_a.push_expr(expr_a.clone());
///
/// assert_eq!("a [a]",format!("{}",expr_a));
/// ```
///
/// # Params
///
/// expr --- The [Expr] to format.  
/// fmt --- The [Formatter] to write to.  
pub fn expr_fmt<A>(expr: &Expr<A>, fmt: &mut Formatter) -> fmt::Result
  where A: Allocator {
  write!(fmt,"{}",expr.head_token)?;
  let mut children = expr.children.iter();
  if let Some(child_expr) = children.next() {
    write!(fmt," [{}",child_expr)?;
    for child_expr in children {
      write!(fmt,", {}",child_expr)?;
    }
    write!(fmt,"]")?;
  }

  Ok(())
}

/// Expression tree of [Tokens][Token].
#[derive(Debug)]
pub struct Expr<A>
  where A: Allocator {
  /// Symbol at the head of this expression.
  pub head_token: Token<A>,
  /// Child expressions.
  pub children: Vec<Self,A>,
  /// A customisable formatting method for [Display].
  pub fmt: fn(&Self, &mut Formatter) -> fmt::Result,
}

impl<A> Expr<A>
  where A: Allocator {
  /// Constructs a new expression from parts.
  ///
  /// # Params
  ///
  /// head_token --- Symbol at the head of this expression.  
  /// children --- Child expressions.  
  /// fmt --- A customisable formatting method for [Display].  
  pub const fn from_parts(head_token: Token<A>, children: Vec<Self,A>,
                          fmt: fn(&Self, &mut Formatter) -> fmt::Result) -> Self {
    Self { head_token, children, fmt }
  }
  /// Constructs a new expression from parts.
  ///
  /// Uses the default formatting method.
  ///
  /// # Params
  ///
  /// head_token --- Symbol at the head of this expression.  
  /// children --- Child expressions.  
  pub const fn new(head_token: Token<A>, children: Vec<Self,A>) -> Self {
    Self::from_parts(head_token,children,expr_fmt)
  }
  /// Converts a [Token] to an [Expr].
  ///
  /// # Params
  ///
  /// token --- Symbol at the head of this expression.  
  /// alloc --- Allocator of the [Expr].  
  pub const fn from_token(token: Token<A>, alloc: A) -> Self { Self::new(token,Vec::new_in(alloc)) }
  /// Adds a child to the end of `self`s children.
  ///
  /// # Params
  ///
  /// expr --- New child to append.  
  pub fn push_expr(&mut self, expr: Self) -> &mut Self {
    self.children.push(expr); self
  }
}

impl<A> Clone for Expr<A>
  where A: Allocator + Clone, {
  fn clone(&self) -> Self {
    Self::from_parts(self.head_token.clone(),self.children.clone(),self.fmt)
  }
}

impl<A> Eq for Expr<A>
  where A: Allocator, {}

impl<A,B> PartialEq<Expr<B>> for Expr<A>
  where A: Allocator, B: Allocator, {
  fn eq(&self, rhs: &Expr<B>) -> bool {
    self.head_token == rhs.head_token && self.children == rhs.children
  }
}

impl<A> Display for Expr<A>
  where A: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { (self.fmt)(self,fmt) }
}

mod tests {
  #![cfg(test)]
  use alloc::alloc::Global;
  use crate::exprs::Token;
  use crate::exprs::builders::Builder;

  #[test]
  fn test_expr_eq() {
    let alloc = Global;

    let tok_a = Token::from_str("a",alloc);
    let tok_b = Token::from_str("b",alloc);
    let tok_expr_a = Builder::from_token(tok_a.clone(),alloc).finish();
    let tok_expr_b = Builder::from_token(tok_b.clone(),alloc).finish();
    assert_eq!(tok_expr_a,tok_expr_a,"`Expr` of a single `Token` is not reflexive");

    let mut builder = Builder::from_token(tok_a.clone(),alloc);
    builder.lens().push_token(tok_a.clone(),alloc);
    let single_child_expr_a = builder.finish();
    assert_eq!(single_child_expr_a,single_child_expr_a,"`Expr` with a single child is not reflexive");

    assert_ne!(tok_expr_a,single_child_expr_a,"`Expr`s with different number of children match");

    assert_ne!(tok_expr_a,tok_expr_b,"`Expr`s with different head tokens match");

    let mut builder = Builder::from_token(tok_a.clone(),alloc);
    builder.lens().push_token(tok_b.clone(),alloc);
    let single_child_expr_b = builder.finish();
    assert_ne!(single_child_expr_a,single_child_expr_b,"`Expr`s with different child tokens match");
  }
}
