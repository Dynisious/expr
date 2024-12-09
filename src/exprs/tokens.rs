//! Defines the [Token] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-06

use alloc::alloc::Allocator;
use alloc::boxed::Box;
use core::fmt::{self,Display,Formatter};

fn copy_str_in<A>(text: &str, alloc: A) -> Box<str,A>
  where A: Allocator {
  let mut dest = unsafe { Box::<[u8],A>::new_uninit_slice_in(text.len(),alloc).assume_init() };

  dest.copy_from_slice(text.as_bytes());
  let (token_ptr,alloc) = Box::into_raw_with_allocator(dest);

  unsafe { Box::from_raw_in(token_ptr as *mut str,alloc) }
}

/// Text token.
#[derive(Debug)]
pub struct Token<A>
  where A: Allocator, {
  /// Text of the Token.
  pub text: Box<str,A>,
}

impl<A> Token<A>
  where A: Allocator {
  /// Constructs a Token from parts.
  ///
  /// # Params
  ///
  /// text --- Text of the Token.  
  pub const unsafe fn from_parts(text: Box<str,A>) -> Self { Self { text } }
  /// Constructs a Token from a [str].
  ///
  /// # Params
  ///
  /// token --- Text of the Token.  
  /// alloc --- Allocator of the Token.  
  pub fn from_str(token: &str, alloc: A) -> Self {
    unsafe { Self::from_parts(copy_str_in(token,alloc)) }
  }
}

impl<A> Clone for Token<A>
  where A: Allocator + Clone, {
  fn clone(&self) -> Self {
    unsafe { Self::from_parts(copy_str_in(&self.text,Box::allocator(&self.text).clone())) }
  }
}

impl<A> Eq for Token<A>
  where A: Allocator {}

impl<A,B> PartialEq<Token<B>> for Token<A>
  where A: Allocator, B: Allocator, {
  fn eq(&self, rhs: &Token<B>) -> bool { *self.text == *rhs.text }
}

impl<A> Display for Token<A>
  where A: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt,"{}",self.text) }
}
