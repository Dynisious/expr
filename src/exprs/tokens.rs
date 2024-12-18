//! Defines the [Token] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-18

use alloc::alloc::Allocator;
use core::fmt::{self,Debug,Display,Formatter};
use crate::nodes::impls::{self,Owned};

/// Text token.
pub struct Token<Alloc>(pub(crate) TokenInner<Owned<Alloc>>)
  where Alloc: Allocator;

/// Internals of a token.
pub(crate) struct TokenInner<Impl>
  where Impl: impls::Impl {
  /// Text of the Token.
  pub text: Impl::Text,
}

impl<Impl> TokenInner<Impl>
  where Impl: impls::Impl {
  /// Constructs a Token from parts.
  ///
  /// # Params
  ///
  /// text --- Text of the Token.  
  pub const fn from_parts(text: Impl::Text) -> Self { Self { text } }
}

impl<Impl> Clone for TokenInner<Impl>
  where Impl: impls::Impl, Impl::Text: Clone {
  fn clone(&self) -> Self { Self::from_parts(self.text.clone()) }
  fn clone_from(&mut self, source: &Self) { self.text.clone_from(&source.text) }
}

impl<Impl> Copy for TokenInner<Impl>
  where Impl: impls::Impl, Impl::Text: Copy {}

impl<Impl> Debug for TokenInner<Impl>
  where Impl: impls::Impl, Impl::Text: Debug {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.text.fmt(fmt) }
}

impl<Impl> Display for TokenInner<Impl>
  where Impl: impls::Impl, Impl::Text: Display {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.text.fmt(fmt) }
}
