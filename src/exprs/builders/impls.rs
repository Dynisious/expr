//! Defines the implementations of the [Node] type for [Builders][Builder].
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-17

#[cfg(doc)]
use crate::nodes::Node;
#[cfg(doc)]
use crate::exprs::builders::Builder;
pub use self::owned::Owned;

mod owned;
