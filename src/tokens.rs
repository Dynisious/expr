//! Defines the [Token] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-25

use alloc::alloc::{Allocator,Global};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::convert::AsRef;
use core::fmt::{self,Debug,Display,Formatter};
use core::{mem,ptr};
use core::str::{self,FromStr};
use core::ops::Deref;

/// Text token.
#[repr(transparent)]
pub struct Token<Alloc>
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
  /// * bytes[..byte.len()] must be valid utf8 text.
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
  /// Gets the token allocator.
  pub fn allocator(&self) -> &Alloc { self.bytes.allocator() }
  /// Gets the token text.
  pub const fn as_str(&self) -> &str {
    unsafe { str::from_utf8_unchecked(self.bytes.as_slice()) }
  }
}

impl<Alloc> Clone for Token<Alloc>
  where Alloc: Allocator + Clone {
  fn clone(&self) -> Self { unsafe { Self::from_parts(self.bytes.clone()) } }
  fn clone_from(&mut self, source: &Self) { self.bytes.clone_from(&source.bytes) }
}

impl From<&str> for Token<Global> {
  fn from(from: &str) -> Self { Self::from_str_in(from,Global) }
}

impl FromStr for Token<Global> {
  type Err = !;

  fn from_str(s: &str) -> Result<Self,Self::Err> { Ok(s.into()) }
}

impl<Alloc> Eq for Token<Alloc>
  where Alloc: Allocator {}

impl<Alloc, Rhs> PartialEq<Rhs> for Token<Alloc>
  where Alloc: Allocator, Rhs: Borrow<str> {
  fn eq(&self, rhs: &Rhs) -> bool { self.as_str() == rhs.borrow() }
}

impl<Alloc> Deref for Token<Alloc>
  where Alloc: Allocator {
  type Target = str;

  fn deref(&self) -> &Self::Target { self.as_str() }
}

impl<Alloc> Borrow<str> for Token<Alloc>
  where Alloc: Allocator {
  fn borrow(&self) -> &str { self.as_str() }
}

impl<Alloc> AsRef<str> for Token<Alloc>
  where Alloc: Allocator {
  fn as_ref(&self) -> &str { self.as_str() }
}

impl<Alloc> Display for Token<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str(self) }
}

impl<Alloc> Debug for Token<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(self,fmt) }
}
