//! Provides representations of expression trees.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-03-16
#![no_std]
#![deny(missing_docs)]
#![feature(allocator_api)]

pub use crate::expr::Expr;

extern crate alloc;
extern crate vec_buf;

pub mod expr;
