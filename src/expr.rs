//! Defines the type of expression trees.
//!
//! Author --- DMorgan  
//! Last Modified --- 2025-03-16

use alloc::alloc::{Allocator,Global};
use vec_buf::Vec;

struct ExprInner<Token> {
  /// `Token` at the head of the expression tree.
  _head_token: Token,
  /// sub-expressions of the expression tree.
  _sub_exprs: Vec<Self>,
}

impl<Token> ExprInner<Token> {
  pub const unsafe fn from_parts(_head_token: Token, _sub_exprs: Vec<Self>) -> Self {
    Self{_head_token,_sub_exprs}
  }
}

/// An owned expression tree.
pub struct Expr<Token,Alloc>
  where Alloc: Allocator {
  /// Root of the expression tree.
  _root_expr: ExprInner<Token>,
  /// Allocator of the expression tree.
  _allocator: Alloc,
}

impl<Token,Alloc> Expr<Token,Alloc>
  where Alloc: Allocator {
  const unsafe fn from_parts(_root_expr: ExprInner<Token>, _allocator: Alloc) -> Self {
    Self{_root_expr,_allocator}
  }
  /// Constructs an Expr from a `Token`.
  ///
  /// # Params
  ///
  /// head_token --- `Token` at the head of the expression tree.  
  /// allocator --- [Allocator] of the expression tree.  
  pub const fn new_in(head_token: Token, allocator: Alloc) -> Self {
    let sub_exprs = Vec::empty();
    let root_expr = unsafe { ExprInner::from_parts(head_token,sub_exprs) };

    unsafe { Self::from_parts(root_expr,allocator) }
  }
}

impl<Token> Expr<Token,Global> {
  /// Constructs an Expr from a `Token`.
  ///
  /// # Params
  ///
  /// head_token --- `Token` at the head of the expression tree.  
  pub const fn new(head_token: Token) -> Self {
    let allocator = Global;

    Self::new_in(head_token,allocator)
  }
}
