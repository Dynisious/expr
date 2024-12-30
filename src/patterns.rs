//! Defines the [Pattern] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-30

use alloc::alloc::Allocator;
use core::fmt::{self,Display,Debug,Formatter};
use core::{mem,ptr};
use crate::exprs::Builder;
use sparse_vec::SparseVec;

/// Formatting method for [Displaying][Display] [Patterns][Pattern].
pub type FmtPattern<Token,Alloc> = fn(pattern: &Pattern<Token,Alloc>, fmt: &mut Formatter) -> fmt::Result;

/// The default `FmtPattern` implementation.
pub fn fmt_pattern<Token,Alloc>(pattern: &Pattern<Token,Alloc>, fmt: &mut Formatter) -> fmt::Result
  where Token: Display, Alloc: Allocator {
  write!(fmt,"{}",pattern.head_token)?;

  let mut child_patterns = pattern.child_patterns.iter();
  if let Some((mut last_index,child)) = child_patterns.next() {
    let dots = if 0 == last_index { "" } else { "... " };

    write!(fmt," [{}{}",dots,child)?;

    for (index, child) in child_patterns {
      let dots = if 1 != index - last_index { "" } else { "...," };

      write!(fmt,",{} {}",dots,child)?;
      last_index = index;
    }
    write!(fmt,"]")?;
  }

  Ok(())
}

/// Pattern matching against [Exprs][Expr].
pub struct Pattern<Token, Alloc>
  where Alloc: Allocator {
  /// Pattern to match against the token at the head of the expression.
  pub head_token: Token,
  /// Child patterns matching against the children of the expression.
  pub child_patterns: SparseVec<Self,Alloc>,
  /// Custom formatting method for [Display].
  pub fmt_pattern: FmtPattern<Token,Alloc>,
}

impl<Token, Alloc> Pattern<Token, Alloc>
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
  /// Holes in `builder` do not match against any pattern except a [PWildcard].
  ///
  /// # Params
  ///
  /// builder --- Partially built [Expr] to match against.  
  pub const fn match_builder<Token1,Alloc1>(&self, _builder: &Builder<Token1, Alloc1>) -> bool
    where Alloc1: Allocator { todo!() }
}

impl<Token, Alloc> Display for Pattern<Token, Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { (self.fmt_pattern)(self,fmt) }
}

impl<Token, Alloc> Debug for Pattern<Token, Alloc>
  where Token: Debug, Alloc: Allocator {
  fn fmt(&self, _fmt: &mut Formatter) -> fmt::Result { todo!() }
}
