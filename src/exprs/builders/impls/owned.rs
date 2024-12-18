//! Defines the [Owned] implementation of the [Node] type for [Builders][Builder].
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-17

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::marker::PhantomData;
use crate::exprs::{Expr,Token};
use crate::exprs::builders::{Builder,BuilderInner,Lens};
use crate::nodes::Node;
use crate::nodes::impls::{self,Impl};
use BuilderInner::*;

/// Owned implementation of a [Node].
pub struct Owned<Alloc>(PhantomData<Alloc>);

impl<Alloc> Impl for Owned<Alloc>
  where Alloc: Allocator {
  type Text = <impls::Owned<Alloc> as Impl>::Text;
  type Token = <impls::Owned<Alloc> as Impl>::Token;
  type Expr = <impls::Owned<Alloc> as Impl>::Expr;
  type Fmt = <impls::Owned<Alloc> as Impl>::Fmt;
  type Children = Vec<Builder<Alloc>,Alloc>;
  type Builder = Self;

  fn can_finish(node: &Node<Self::Builder>) -> bool {
    let children = node.children.as_slice();
    let mut index = 0;
    while index < children.len() {
      let child = &children[index];

      if !child.can_finish() { return false }
      index += 1;
    }

    true
  }
  fn finish(node: Node<Self::Builder>) -> Self::Expr {
    use map_in_place::vec::alloc;

    let (head_token,children,fmt) = node.into_inner();
    let children = alloc::map(children,Builder::finish);

    Expr::from_parts(head_token,children,fmt)
  }
}

impl<Alloc> Node<Owned<Alloc>>
  where Alloc: Allocator {
// TODO: Uncomment once implemented.
//
//   /// Constructs a new expression from parts.
//   ///
//   /// Uses the default formatting method.
//   ///
//   /// # Params
//   ///
//   /// head_token --- Symbol at the head of this expression.  
//   /// children --- Child expressions.  
//   pub const fn new(head_token: Token<Owned<Alloc>>, children: <Owned<Alloc> as Impl>::Children) -> Self {
//     let fmt = crate::exprs::fmt_expr;
// 
//     Self::from_parts(head_token,children,fmt)
//   }
  /// Converts a complete [Expr] to a partially built [Expr].
  ///
  /// # Params
  ///
  /// expr --- Constituting [Expr].  
  pub fn from_expr(expr: Expr<Alloc>) -> Self {
    use map_in_place::vec::alloc;

    let (head_token,children,fmt) = expr.into_inner();
    let children = alloc::map(children,Builder::from_expr);

    Self::from_parts(head_token,children,fmt)
  }
// TODO: Uncomment once implemented.
//
//   /// Converts a [Token] to an [Expr].
//   ///
//   /// # Params
//   ///
//   /// token --- Symbol at the head of this expression.  
//   /// alloc --- [Allocator] of the [Expr].  
//   pub const fn from_token(token: Token<Owned<Alloc>>, alloc: Alloc) -> Self {
//     Self::new(token,Vec::new_in(alloc))
//   }
//   /// Converts a [str] to an [Expr].
//   ///
//   /// # Params
//   ///
//   /// text --- Symbol at the head of this expression.  
//   /// alloc --- [Allocator] of the [Expr].  
//   pub fn from_str(text: &str, alloc: Alloc) -> Self
//     where Alloc: Clone {
//     Self::from_token(Token::<Owned<Alloc>>::from_str(text,alloc.clone()),alloc)
//   }
  /// Adds a child to the end of `self`s children.
  ///
  /// # Params
  ///
  /// builder --- New child to append.  
  pub fn push_builder(&mut self, builder: Builder<Alloc>) -> &mut Self {
    self.children.push(builder); self
  }
// TODO: Uncomment once implemented.
//
//   /// Adds a child to the end of `self`s children.
//   ///
//   /// # Params
//   ///
//   /// expr --- New child to append.  
//   pub fn push_expr(&mut self, expr: Expr<Alloc>) -> &mut Self {
//     self.push_builder(Builder::from_expr(expr))
//   }
//   /// Tests that there are no holes in the [Expr].
//   pub const fn can_finish(&self) -> bool {
//     let mut index = 0;
//     while index < self.children.len() {
//       let child = &self.children.as_slice()[index];
//       index += 1;
// 
//       if !child.can_finish() { return false }
//     }
// 
//     true
//   }
//   /// Returns the built [Expr].
//   ///
//   /// # Panics
//   ///
//   /// If the [Expr] has unfilled holes; use [can_finish][Self::can_finish] to test if this
//   /// function would panic.
//   pub fn finish(self) -> Expr<Alloc> {
//     use core::ptr;
//     use map_in_place::vec::alloc;
// 
//     let self_ = ManuallyDrop::new(self);
//     let head_token = {
//         let Token { text } = unsafe { ptr::read(&self_.head_token) };
// 
//         Token::from_parts(text)
//       };
//     let children = alloc::map(unsafe { ptr::read(&self_.children) },Builder::finish);
// 
//     Expr(Node::from_parts(head_token,children,self_.fmt))
//   }
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

impl<Alloc> Builder<Alloc>
  where Alloc: Allocator {
  /// Construct a Builder which is a hole to be filled.
  pub const fn hole() -> Self { Builder(BHole) }
  /// Constructs a Builder from an [Expr].
  ///
  /// # Params
  ///
  /// expr --- Constituting [Expr].  
  pub const fn from_expr(expr: Expr<Alloc>) -> Self { Builder(BuilderInner::from_expr(expr)) }
  /// Constructs a Builder from a partial [Expr].
  ///
  /// # Params
  ///
  /// node --- Partially built [Expr].  
  pub(crate) const fn from_node(node: Node<Owned<Alloc>>) -> Self {
    Builder(BuilderInner::from_node(node))
  }
  /// Constructs a Builder from a [Token].
  ///
  /// # Params
  ///
  /// token --- Constituting [Token].  
  /// alloc --- [Allocator] of `token`.  
  pub fn from_token(token: Token<Alloc>, alloc: Alloc) -> Self {
    Self::from_expr(Expr::from_token(token,alloc))
  }
  /// Constructs a Builder from a token.
  ///
  /// # Params
  ///
  /// token --- Constituting [Token].  
  /// alloc --- [Allocator] of `token`.  
  pub fn from_str(token: &str, alloc: Alloc) -> Self
    where Alloc: Clone {
    Self::from_token(Token::from_str(token,alloc.clone()),alloc)
  }
  /// Constructs a [Lens] pointing at `self`.
  pub const fn lens(&mut self) -> Lens<Alloc> { Lens::from_builder(self) }
  /// Tests that there are no holes in the [Expr].
  ///
  /// # Examples
  ///
  /// ```
  /// # #![feature(allocator_api)]
  /// #
  /// # extern crate alloc;
  /// #
  /// # use alloc::alloc::Global;
  /// # use expr::exprs::{Expr,Token};
  /// # use expr::exprs::builders::Builder;
  /// #
  /// # let token_a = Token::from_str("a",Global);
  /// let mut builder = Builder::hole();
  ///
  /// assert!(!builder.can_finish());
  /// 
  /// builder.lens().fill_token(token_a.clone(),Global);
  /// assert!(builder.can_finish());
  ///
  /// builder.lens().push_token(token_a.clone(),Global);
  /// assert!(builder.can_finish());
  ///
  /// builder.lens().push_builder(Builder::hole());
  /// assert!(!builder.can_finish());
  /// ```
  pub fn can_finish(&self) -> bool { self.0.can_finish() }
  /// Returns the built [Expr].
  ///
  /// # Examples
  ///
  /// ```
  /// # #![feature(allocator_api)]
  /// #
  /// # extern crate alloc;
  /// #
  /// # use alloc::alloc::Global;
  /// # use expr::exprs::{Expr,Token};
  /// # use expr::exprs::builders::Builder;
  /// #
  /// # let token_a = Token::from_str("a",Global);
  /// # let token_b = Token::from_str("b",Global);
  /// let mut builder = Builder::from_token(token_a.clone(),Global);
  ///
  /// assert_eq!("a",format!("{}",builder.clone().finish()));
  ///
  /// builder.lens().push_token(token_a.clone(),Global);
  /// assert_eq!("a [a]",format!("{}",builder.clone().finish()));
  ///
  /// let mut lens = builder.lens();
  /// lens.push_builder(Builder::hole());
  /// lens.visit_child(1).fill_token(token_b.clone(),Global);
  /// assert_eq!("a [a, b]",format!("{}",builder.clone().finish()));
  /// ```
  ///
  /// # Panics
  ///
  /// If the [Expr] has unfilled holes; use [can_finish][Self::can_finish] to test if this
  /// function would panic.
  pub fn finish(self) -> Expr<Alloc> {
    debug_assert!(self.can_finish(),"called `finish` on a `Builder` with holes");
  
    self.0.finish()
  }
}

impl<Alloc> Clone for Builder<Alloc>
  where Alloc: Allocator + Clone {
  fn clone(&self) -> Self {
    match &self.0 {
      BHole       => Self::hole(),
      BExpr(expr) => Self::from_expr(expr.clone()),
      BNode(node) => Builder(BuilderInner::from_node(node.clone())),
    }
  }
  fn clone_from(&mut self, source: &Self) {
    match (&mut self.0,&source.0) {
      (BExpr(expr), BExpr(source)) => expr.clone_from(&source),
      (BNode(node), BNode(source)) => node.clone_from(&source),
      (_          , _)             => *self = source.clone(),
    }
  }
}

impl<'a,Alloc> Lens<'a,Alloc>
  where Alloc: Allocator {
  /// Constructs a new Lens pointing at the root of the [Expr] under construction by `builder`.
  ///
  /// # Params
  ///
  /// builder --- The [Builder] to point into.  
  pub const fn from_builder(builder: &'a mut Builder<Alloc>) -> Self { Lens(builder) }
  /// Tests that `self` is pointing to a hole.
  pub const fn is_hole(&self) -> bool {
    match self.0.0 {
      BHole               => true,
      BExpr(_) | BNode(_) => false,
    }
  }
  /// Replaces the [Expr] being pointed at with `builder`.
  ///
  /// Returns the [Expr] that was being pointed at.
  ///
  /// # Params
  ///
  /// builder --- [Builder] to fill the hole with.  
  pub const fn replace_builder(&mut self, builder: Builder<Alloc>) -> Builder<Alloc> {
    use core::mem;

    mem::replace(self.0,builder)
  }
  /// Fills the hole with `builder`.
  ///
  /// Returns a Lens pointing at `builder`.
  ///
  /// # Params
  ///
  /// builder --- [Builder] to fill the hole with.  
  ///
  /// # Panics
  ///
  /// If this is not a hole; use [is_hole][Lens::is_hole] to check.
  pub fn fill_builder(&mut self, builder: Builder<Alloc>) -> &mut Self {
    match self.0.0 {
      BHole => { *self.0 = builder; self },
      _     => panic!("attempted to fill a non-hole"),
    }
  }
  /// Fills the hole with `expr`.
  ///
  /// Returns a Lens pointing at `expr`.
  ///
  /// # Params
  ///
  /// expr --- [Expr] to fill the hole with.  
  ///
  /// # Panics
  ///
  /// If this is not a hole; use [is_hole][Lens::is_hole] to check.
  pub fn fill_expr(&mut self, expr: Expr<Alloc>) -> &mut Self {
    self.fill_builder(Builder::from_expr(expr))
  }
  /// Fills the hole with `token`.
  ///
  /// Returns a Lens pointing at `token`.
  ///
  /// # Params
  ///
  /// token --- [Token] to fill the hole with.  
  /// alloc --- [Allocator] of the [Token].  
  ///
  /// # Panics
  ///
  /// If this is not a hole; use [is_hole][Lens::is_hole] to check.
  pub fn fill_token(&mut self, token: Token<Alloc>, alloc: Alloc) -> &mut Self {
    self.fill_expr(Expr::from_token(token,alloc))
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// builder --- New child to push.  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_builder(&mut self, builder: Builder<Alloc>) -> &mut Self {
    use core::mem;

    match &mut self.0 {
      lens@Builder(BExpr(_)) => {
          let Builder(BExpr(expr)) = mem::replace(*lens,Builder::hole())
            else { if cfg!(debug_assertions) { unreachable!("matched `self` as `BExpr`") }
                    else { unsafe { core::hint::unreachable_unchecked() } } };
          let mut node = Node::from_expr(expr);

          node.push_builder(builder);
          **lens = Builder::from_node(node);
          self
        },
      Builder(BHole)       => panic!("attempted to add a child to a hole"),
      Builder(BNode(node)) => { node.push_builder(builder); self },
    }
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// expr --- New child to push.  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_expr(&mut self, expr: Expr<Alloc>) -> &mut Self {
    match &mut self.0.0 {
      BExpr(lens) => { lens.push_expr(expr); self },
      _           => self.push_builder(Builder::from_expr(expr)),
    }
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// token --- New child to push.  
  /// alloc --- [Allocator] of the [Expr].  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_token(&mut self, token: Token<Alloc>, alloc: Alloc) -> &mut Self {
    self.push_expr(Expr::from_token(token,alloc))
  }
  /// Adds a child to the [Expr].
  ///
  /// Returns a Lens pointing at the same position as `self`.
  ///
  /// # Params
  ///
  /// token --- New child to push.  
  /// alloc --- [Allocator] of the [Expr].  
  ///
  /// # Panics
  ///
  /// If this is a hole; use [is_hole][Lens::is_hole] to check.
  pub fn push_str(&mut self, token: &str, alloc: Alloc) -> &mut Self
    where Alloc: Clone {
    self.push_expr(Expr::from_str(token,alloc))
  }
  /// Returns a Lens pointing at the child at `child_index`.
  ///
  /// # Params
  ///
  /// child_index --- Index of the child node to index.  
  ///
  /// # Panics
  ///
  /// * `self` is pointing to a hole.
  /// * `child_index` is not in bounds.
  pub fn visit_child(&'a mut self, child_index: usize) -> Self {
    use core::mem;

    let builder = match &mut self.0 {
        Builder(BHole)         => panic!("attempted to visit a child of a hole"),
        Builder(BNode(node))   => &mut node.children[child_index],
        lens@Builder(BExpr(_)) => {
            let Builder(BExpr(expr)) = mem::replace(*lens,Builder::hole())
              else { if cfg!(debug_assertions) { unreachable!("matched `self` as `BExpr`") }
                     else { unsafe { core::hint::unreachable_unchecked() } } };
            **lens = Builder::from_node(Node::from_expr(expr));
            let Builder(BNode(node)) = lens
              else { if cfg!(debug_assertions) { unreachable!("matched `self` as `BExpr`") }
                     else { unsafe { core::hint::unreachable_unchecked() } } };

            &mut node.children[child_index]
          },
      };

    Self::from_builder(builder)
  }
}
