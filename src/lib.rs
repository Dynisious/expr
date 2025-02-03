//! A runtime for manipulating expressions in a natural deduction style.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-02-03
#![no_std]
#![deny(missing_docs)]
#![feature(allocator_api,box_vec_non_null,const_deref,const_trait_impl,const_vec_string_slice,
           never_type)]

extern crate alloc;
extern crate map_in_place;
extern crate sparse_vec;

pub mod exprs;
pub mod tokens;
pub mod patterns;
