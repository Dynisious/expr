//! Defines the [Pattern] trait.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

pub use self::{eq_patterns::*,expr_patterns::*,wildcard_patterns::*};

mod eq_patterns;
mod expr_patterns;
mod wildcard_patterns;

/// A pattern against `T`s.
#[const_trait]
pub trait Pattern<T> {
  /// Match the pattern against `target`.
  fn match_pattern(&self, target: &T) -> bool;
}
