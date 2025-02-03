//! Defines the [EqPattern] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

use core::fmt::{self,Display,Debug,Formatter};
use crate::patterns::Pattern;

/// Pattern matching via [PartialEq].
pub struct EqPattern<Pattern>(pub Pattern);

impl<PatternT,Rhs> Pattern<Rhs> for EqPattern<PatternT>
  where PatternT: PartialEq<Rhs> {
  fn match_pattern(&self, rhs: &Rhs) -> bool { self.0 == *rhs }
}

impl<Pattern,Rhs> PartialEq<Rhs> for EqPattern<Pattern>
  where Pattern: PartialEq<Rhs> { fn eq(&self, rhs: &Rhs) -> bool { self.0 == *rhs } }

impl<Pattern> Display for EqPattern<Pattern>
  where Pattern: Display {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.0.fmt(fmt) }
}

impl<Pattern> Debug for EqPattern<Pattern>
  where Pattern: Debug {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    fmt.debug_tuple("EqPattern").field(&self.0).finish()
  }
}
