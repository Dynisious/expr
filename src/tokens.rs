//! Defines the [Token] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03

use alloc::alloc::{Allocator,Global};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::convert::AsRef;
use core::fmt::{self,Debug,Display,Formatter};
use core::{mem,ptr};
use core::str::{self,FromStr,Utf8Error};
use core::ops::{Deref,DerefMut};

/// Text token.
#[derive(Clone)]
#[repr(transparent)]
pub struct Token<Alloc = Global>
  where Alloc: Allocator {
  /// Backing bytes of the text.
  bytes: Vec<u8,Alloc>,
}

impl<Alloc> Token<Alloc>
  where Alloc: Allocator {
  /// Constructs a Token from parts.
  ///
  /// # Params
  ///
  /// bytes --- Backing bytes of the text.  
  ///
  /// # Safety
  ///
  /// * bytes[..bytes.len()] must be valid utf8 text.
  pub const unsafe fn from_parts(bytes: Vec<u8, Alloc>) -> Self { Self { bytes } }
  /// Deconstructs a Token into parts.
  pub const fn into_parts(self) -> Vec<u8, Alloc> {
    let bytes = unsafe { ptr::read(&self.bytes) };

    mem::forget(self);
    bytes
  }
  /// Constructs a Token from text.
  ///
  /// # Params
  ///
  /// token --- Text of the Token.  
  /// allocator --- Allocator of the Token.  
  pub fn from_str_in(token: &str, allocator: Alloc) -> Self {
    let mut bytes = Vec::with_capacity_in(token.len(),allocator);
    bytes.extend(token.as_bytes());

    unsafe { Self::from_parts(bytes) }
  }
  /// Constructs a Token from text.
  ///
  /// # Params
  ///
  /// token --- Text of the Token.  
  pub fn from_str(token: &str) -> Self
    where Alloc: Default {
    let alloc = Alloc::default();

    Self::from_str_in(token,alloc)
  }
  /// Gets the token allocator.
  pub fn allocator(&self) -> &Alloc { self.bytes.allocator() }
  /// Gets the token text.
  pub const fn as_str(&self) -> &str {
    unsafe { str::from_utf8_unchecked(self.bytes.as_slice()) }
  }
  /// Gets the token text.
  pub const fn as_str_mut(&mut self) -> &mut str {
    unsafe { str::from_utf8_unchecked_mut(self.bytes.as_mut_slice()) }
  }
  /// Replaces `self[begin..end]` with `text`.
  ///
  /// Returns `None` if `self[begin..end]` is not valid `utf8` text.
  pub fn splice_checked(&mut self, begin: usize, end: usize, text: &str) -> Option<&mut Self> {
    //Check that `self[begin..end]` is valid utf8 text.
    self.as_str().split_at_checked(end)?.0.split_at_checked(begin)?;

    self.bytes.splice(begin..end,text.as_bytes().iter().copied());
    Some(self)
  }
  /// Replaces `self[begin..end]` with `text`.
  ///
  /// # Panics
  ///
  /// * If `self[begin..end]` is not valid `utf8` text.
  #[track_caller]
  pub fn splice(&mut self, begin: usize, end: usize, text: &str) -> &mut Self {
    self.splice_checked(begin,end,text).unwrap()
  }
}

impl<Alloc> From<&str> for Token<Alloc>
  where Alloc: Allocator + Default {
  fn from(from: &str) -> Self { Self::from_str(from) }
}

impl<Alloc> TryFrom<&[u8]> for Token<Alloc>
  where Alloc: Allocator + Default {
  type Error = Utf8Error;

  /// Succeeds if `from` is valid `utf8` text.
  fn try_from(from: &[u8]) -> Result<Self,Self::Error> { Ok(str::from_utf8(from)?.into()) }
}

impl<Alloc> FromStr for Token<Alloc>
  where Alloc: Allocator + Default {
  type Err = !;

  fn from_str(s: &str) -> Result<Self,Self::Err> { Ok(s.into()) }
}

impl<Alloc> Eq for Token<Alloc>
  where Alloc: Allocator {}

impl<Alloc, Rhs> PartialEq<Rhs> for Token<Alloc>
  where Alloc: Allocator, Rhs: Borrow<str> {
  fn eq(&self, rhs: &Rhs) -> bool { self.as_str() == rhs.borrow() }
}

impl<Alloc> const Deref for Token<Alloc>
  where Alloc: Allocator {
  type Target = str;

  fn deref(&self) -> &Self::Target { self.as_str() }
}

impl<Alloc> const DerefMut for Token<Alloc>
  where Alloc: Allocator {
  fn deref_mut(&mut self) -> &mut Self::Target { self.as_str_mut() }
}

impl<Alloc> Borrow<str> for Token<Alloc>
  where Alloc: Allocator {
  fn borrow(&self) -> &str { self.as_str() }
}

impl<T,Alloc> Borrow<T> for Token<Alloc>
  where str: Borrow<T>, Alloc: Allocator {
  fn borrow(&self) -> &T { self.as_str().borrow() }
}

impl<Alloc> AsRef<Self> for Token<Alloc>
  where Alloc: Allocator {
  fn as_ref(&self) -> &Self { self }
}

impl<T,Alloc> AsRef<T> for Token<Alloc>
  where str: AsRef<T>, Alloc: Allocator {
  fn as_ref(&self) -> &T { self.as_str().as_ref() }
}

impl<Alloc> Display for Token<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str(self) }
}

impl<Alloc> Debug for Token<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.as_str(),fmt) }
}

impl<Alloc> PartialEq<Token<Alloc>> for str
  where Alloc: Allocator {
  fn eq(&self, rhs: &Token<Alloc>) -> bool { self == rhs.as_str() }
}

mod tests {
  #![cfg(test)]
  use alloc::alloc::Global;
  use crate::tokens::Token;

  #[test]
  fn test_token_eq() {
    let alloc = Global;

    let tok_a = Token::from_str_in("a",alloc);
    assert_eq!(tok_a,tok_a,"`Expr` of a single `Token` is not reflexive");

    let tok_b = Token::from_str_in("b",alloc);
    assert_ne!(tok_a,tok_b,"`Expr`s with different head tokens match");
  }
}
