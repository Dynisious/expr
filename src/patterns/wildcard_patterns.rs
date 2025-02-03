//! Defines the [WildcardPattern] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

use core::fmt::{self,Display,Debug,Formatter};
use crate::patterns::Pattern;

/// Wildcard pattern which matches against everything.
pub struct WildcardPattern;

impl<Token> const Pattern<Token> for WildcardPattern {
  fn match_pattern(&self, _rhs: &Token) -> bool { true }
}

impl<Token> PartialEq<Token> for WildcardPattern {
  fn eq(&self, token: &Token) -> bool { self.match_pattern(token) }
}

impl Display for WildcardPattern {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt,"Wildcard") }
}

impl Debug for WildcardPattern {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self,fmt) }
}
