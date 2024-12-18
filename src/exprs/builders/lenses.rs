//! Defines the [Lens] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-18

use alloc::alloc::Allocator;
#[cfg(doc)]
use crate::exprs::Expr;
use crate::exprs::builders::Builder;

/// View into the [Expr] under construction by a [Builder].
pub struct Lens<'a,Alloc>(pub(crate) &'a mut Builder<Alloc>)
  where Alloc: Allocator;
