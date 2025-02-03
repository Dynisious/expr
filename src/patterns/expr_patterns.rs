//! Defines the [ExprPattern] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

use alloc::alloc::Allocator;
use core::fmt::{self,Display,Debug,Formatter};
use core::{mem,ptr};
use crate::exprs::{Builder,Expr};
use crate::patterns::Pattern;
use sparse_vec::SparseVec;
use Builder::*;

/// Formatting method for [Displaying][Display] [Patterns][Pattern].
pub type FmtPattern<Token,Alloc> = fn(pattern: &ExprPattern<Token,Alloc>, fmt: &mut Formatter
                                     ) -> fmt::Result;

/// The default `FmtPattern` implementation.
pub fn fmt_pattern<Token,Alloc>(pattern: &ExprPattern<Token,Alloc>, fmt: &mut Formatter) -> fmt::Result
  where Token: Display, Alloc: Allocator {
  write!(fmt,"{}",pattern.head_token)?;

  let mut child_patterns = pattern.child_patterns.iter();
  if let Some((mut last_index,child)) = child_patterns.next() {
    let dots = if 0 == last_index { "" } else { "... " };

    write!(fmt," [{}{}",dots,child)?;

    for (index, child) in child_patterns {
      let dots = if 1 == index - last_index { "" } else { "...," };

      write!(fmt,",{} {}",dots,child)?;
      last_index = index;
    }
    write!(fmt,"]")?;
  }

  Ok(())
}

/// Pattern matching against [Exprs][Expr].
pub struct ExprPattern<Token, Alloc>
  where Alloc: Allocator {
  /// Pattern to match against the token at the head of the expression.
  pub head_token: Token,
  /// Child patterns matching against the children of the expression.
  pub child_patterns: SparseVec<Self,Alloc>,
  /// Custom formatting method for [Display].
  pub fmt_pattern: FmtPattern<Token,Alloc>,
}

impl<Token, Alloc> ExprPattern<Token, Alloc>
  where Alloc: Allocator {
  /// Deconstruct `self` into parts.
  ///
  /// Post-inverse of `from_parts`.
  pub const fn into_parts(self) -> (Token, SparseVec<Self,Alloc>, FmtPattern<Token,Alloc>) {
    let head_token = unsafe { ptr::read(&self.head_token) };
    let child_patterns = unsafe { ptr::read(&self.child_patterns) };
    let fmt_pattern = unsafe { ptr::read(&self.fmt_pattern) };

    mem::forget(self);
    (head_token,child_patterns,fmt_pattern)
  }
  /// Constructs a Pattern from parts.
  ///
  /// # Params
  ///
  /// head_token --- Pattern to match against the token at the head of the expression.  
  /// child_patterns --- Child patterns matching against the children of the expression.  
  /// fmt_pattern --- Custom formatting method for [Display].  
  pub const fn from_parts(head_token: Token, child_patterns: SparseVec<Self,Alloc>,
                          fmt_pattern: FmtPattern<Token,Alloc>) -> Self {
    Self{head_token,child_patterns,fmt_pattern}
  }
  /// Constructs a Pattern from a token pattern.
  ///
  /// # Params
  ///
  /// head_token --- Pattern to match against the token at the head of the expression.  
  /// allocator --- [Allocator] of child patterns.  
  pub const fn from_token_in(head_token: Token, allocator: Alloc) -> Self
    where Token: Display {
    let child_patterns = SparseVec::new_in(allocator);

    Self::from_parts(head_token,child_patterns,fmt_pattern)
  }
  /// Checks the [Expr] under construction by `builder` against `self`.
  ///
  /// `BTokenHole` will match `self.head_token` against `()`.
  ///
  /// # Params
  ///
  /// builder --- Partially built [Expr] to match against.  
  pub fn match_builder<Token1,Alloc1>(&self, builder: &Builder<Token1, Alloc1>) -> bool
    where Alloc1: Allocator, Token: Pattern<Token1> + Pattern<()> {
    //Child Builders to be matched against
    let child_builders = match builder {
        BHole                      => return false,
        BTokenHole{child_exprs,..} =>
          if self.head_token.match_pattern(&()) { child_exprs } else { return false },
        BExpr(expr)    => return self.match_expr(expr),
        BPart(builder) =>
          if self.head_token.match_pattern(&builder.head_token) { &builder.child_exprs }
          else { return false },
      };

    self.child_patterns.iter()
      .all(|(index,lhs_child)| child_builders.get(index)
                               //`false` if there is no child to match against
                               .map_or(false,|rhs_child| lhs_child.match_builder(rhs_child)))
  }
  /// Checks the `expr` against `self`.
  ///
  /// # Params
  ///
  /// expr --- [Expr] to match against.  
  pub fn match_expr<Token1,Alloc1>(&self, expr: &Expr<Token1, Alloc1>) -> bool
    where Alloc1: Allocator, Token: Pattern<Token1> {
    self.head_token.match_pattern(&expr.head_token) &&
      self.child_patterns.iter()
        .all(|(index,lhs_child)| expr.child_exprs.get(index)
                                 //`false` if there is no child to match against
                                 .map_or(false,|rhs_child| lhs_child.match_expr(rhs_child)))
  }
  /// Checks the `token` against `self`.
  ///
  /// # Params
  ///
  /// token --- Token to match against.  
  pub const fn match_token<Token1>(&self, token: &Token1) -> bool
    where Token: ~const Pattern<Token1> {
    self.child_patterns.is_empty() && self.head_token.match_pattern(token)
  }
}

impl<Token1, Alloc1, Token2, Alloc2> Pattern<Builder<Token2, Alloc2>> for ExprPattern<Token1, Alloc1>
  where Token1: Pattern<Token2> + Pattern<()>, Alloc1: Allocator, Alloc2: Allocator {
  fn match_pattern(&self, builder: &Builder<Token2, Alloc2>) -> bool { self.match_builder(builder) }
}

impl<Token1, Alloc1, Token2, Alloc2> Pattern<Expr<Token2, Alloc2>> for ExprPattern<Token1, Alloc1>
  where Token1: Pattern<Token2>, Alloc1: Allocator, Alloc2: Allocator {
  fn match_pattern(&self, expr: &Expr<Token2, Alloc2>) -> bool { self.match_expr(expr) }
}

impl<Token1, Alloc1, Token2, Alloc2> PartialEq<Builder<Token2, Alloc2>> for ExprPattern<Token1, Alloc1>
  where Token1: Pattern<Token2> + Pattern<()>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, builder: &Builder<Token2, Alloc2>) -> bool { self.match_pattern(builder) }
}

impl<Token1, Alloc1, Token2, Alloc2> PartialEq<Expr<Token2, Alloc2>> for ExprPattern<Token1, Alloc1>
  where Token1: Pattern<Token2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, expr: &Expr<Token2, Alloc2>) -> bool { self.match_pattern(expr) }
}

impl<Token, Alloc> Display for ExprPattern<Token, Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { (self.fmt_pattern)(self,fmt) }
}

impl<Token, Alloc> Debug for ExprPattern<Token, Alloc>
  where Token: Debug, Alloc: Allocator {
  fn fmt(&self, _fmt: &mut Formatter) -> fmt::Result { todo!() }
}
