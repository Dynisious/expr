//! Defines the [Owned] implementation of the [Node] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-17

use alloc::alloc::Allocator;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::{self,Debug,Display,Formatter};
use core::marker::PhantomData;
use crate::exprs::{Expr,Token};
use crate::exprs::tokens::TokenInner;
use crate::exprs::builders::impls;
use crate::nodes::{self,Node};
use crate::nodes::impls::Impl;

/// Default [Display] formatting method for [Expr].
pub fn fmt_expr<Alloc>(Expr(expr): &Expr<Alloc>, fmt: &mut Formatter) -> fmt::Result
  where Alloc: Allocator {
  let children = expr.children.iter();

  nodes::fmt_node(&expr.head_token,children,fmt)
}

fn copy_str_in<A>(text: &str, alloc: A) -> Box<str,A>
  where A: Allocator {
  let mut dest = unsafe { Box::<[u8],A>::new_uninit_slice_in(text.len(),alloc).assume_init() };

  dest.copy_from_slice(text.as_bytes());
  let (token_ptr,alloc) = Box::into_raw_with_allocator(dest);

  unsafe { Box::from_raw_in(token_ptr as *mut str,alloc) }
}

/// Owned implementation of a [Node].
pub struct Owned<Alloc>(PhantomData<Alloc>);

impl<Alloc> Impl for Owned<Alloc>
  where Alloc: Allocator {
  type Text = Box<str,Alloc>;
  type Fmt = fn(expr: &Expr<Alloc>, fmt: &mut Formatter) -> fmt::Result;
  type Token = Token<Alloc>;
  type Expr = Expr<Alloc>;
  type Children = Vec<Expr<Alloc>,Alloc>;
  type Builder = impls::Owned<Alloc>;

  fn can_finish(node: &Node<Self::Builder>) -> bool { Self::Builder::can_finish(node) }
  fn finish(node: Node<Self::Builder>) -> Self::Expr { Self::Builder::finish(node) }
}

impl<Alloc> Token<Alloc>
  where Alloc: Allocator {
  /// Constructs a Token from a [str].
  ///
  /// # Params
  ///
  /// token --- Text of the Token.  
  /// alloc --- [Allocator] of the Token.
  pub fn from_str(token: &str, alloc: Alloc) -> Self {
    Token(TokenInner::from_parts(copy_str_in(token,alloc)))
  }
}

impl<AllocA,AllocB> PartialEq<Token<AllocB>> for Token<AllocA>
  where AllocA: Allocator, AllocB: Allocator {
  fn eq(&self, rhs: &Token<AllocB>) -> bool { *self.0.text == *rhs.0.text }
}

impl<Alloc> Eq for Token<Alloc>
  where Alloc: Allocator {}

impl<Alloc> Clone for Token<Alloc>
  where Alloc: Allocator + Clone {
  fn clone(&self) -> Self {
    let alloc = Box::allocator(&self.0.text).clone();

    Self::from_str(&self.0.text,alloc)
  }
}

impl<Alloc> AsRef<TokenInner<Owned<Alloc>>> for Token<Alloc>
  where Alloc: Allocator {
  fn as_ref(&self) -> &TokenInner<Owned<Alloc>> { &self.0 }
}

impl<Alloc> Clone for Node<Owned<Alloc>>
  where Alloc: Allocator + Clone {
  fn clone(&self) -> Self {
    let head_token = self.head_token.clone();
    let children = self.children.clone();
    let fmt = self.fmt.clone();

    Self::from_parts(head_token,children,fmt)
  }
  fn clone_from(&mut self, source: &Self) {
    self.head_token.clone_from(&source.head_token);
    self.children.clone_from(&source.children);
    self.fmt.clone_from(&source.fmt);
  }
}

impl<Alloc> Display for Token<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.0,fmt) }
}

impl<Alloc> Debug for Token<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.0,fmt) }
}

impl<Alloc> Expr<Alloc>
  where Alloc: Allocator {
  /// Deconstructs into parts.
  pub fn into_inner(self) -> (Token<Alloc>,Vec<Self,Alloc>,
                              fn(expr: &Expr<Alloc>, fmt: &mut Formatter) -> fmt::Result) {
    self.0.into_inner()
  }
  /// Constructs a new expression from parts.
  ///
  /// # Params
  ///
  /// head_token --- Symbol at the head of this expression.  
  /// children --- Child expressions.  
  /// fmt --- A customisable formatting method for [Display].  
  pub const fn from_parts(head_token: Token<Alloc>, children: Vec<Self,Alloc>,
                          fmt: fn(expr: &Self, fmt: &mut Formatter) -> fmt::Result) -> Self {
    Expr(Node::from_parts(head_token,children,fmt))
  }
  /// Constructs a new expression from parts.
  ///
  /// Uses the default formatting method.
  ///
  /// # Params
  ///
  /// head_token --- Symbol at the head of this expression.  
  /// children --- Child expressions.  
  pub const fn new(head_token: Token<Alloc>, children: Vec<Self,Alloc>) -> Self {
    Self::from_parts(head_token,children,fmt_expr)
  }
  /// Converts a [Token] to an [Expr].
  ///
  /// # Params
  ///
  /// token --- Symbol at the head of this expression.  
  /// alloc --- [Allocator] of the [Expr].  
  pub const fn from_token(token: Token<Alloc>, alloc: Alloc) -> Self {
    Self::new(token,Vec::new_in(alloc))
  }
  /// Converts a [str] to an [Expr].
  ///
  /// # Params
  ///
  /// text --- Symbol at the head of this expression.  
  /// alloc --- [Allocator] of the [Expr].  
  pub fn from_str(text: &str, alloc: Alloc) -> Self
    where Alloc: Clone {
    Self::from_token(Token::from_str(text,alloc.clone()),alloc)
  }
  /// Adds a child to the end of `self`s children.
  ///
  /// # Params
  ///
  /// expr --- New child to append.  
  pub fn push_expr(&mut self, expr: Self) -> &mut Self { self.0.children.push(expr); self }
}

impl<AllocA,AllocB> PartialEq<Expr<AllocB>> for Expr<AllocA>
  where AllocA: Allocator, AllocB: Allocator {
  fn eq(&self, rhs: &Expr<AllocB>) -> bool {
    self.0.head_token == rhs.0.head_token && self.0.children == rhs.0.children
  }
}

impl<Alloc> Clone for Expr<Alloc>
  where Node<Owned<Alloc>>: Clone, Alloc: Allocator {
  fn clone(&self) -> Self { Expr(self.0.clone()) }
  fn clone_from(&mut self, source: &Self) { self.0.clone_from(source.as_ref()) }
}

impl<Alloc> AsRef<Node<Owned<Alloc>>> for Expr<Alloc>
  where Alloc: Allocator {
  fn as_ref(&self) -> &Node<Owned<Alloc>> { &self.0 }
}

impl<Alloc> Display for Expr<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { (self.0.fmt)(self,fmt) }
}

impl<Alloc> Debug for Expr<Alloc>
  where Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    fmt.debug_struct("Expr")
      .field("head_token",&self.0.head_token)
      .field("children",&self.0.children)
      .finish()
  }
}
