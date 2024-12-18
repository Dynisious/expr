//! A runtime for manipulating expressions in a natural deduction style.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-18
#![no_std]
#![deny(missing_docs)]
#![feature(allocator_api,const_vec_string_slice,const_trait_impl,iter_collect_into)]

pub use nodes::fmt_node;

extern crate alloc;
extern crate map_in_place;

pub mod exprs;
mod nodes;
