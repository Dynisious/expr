//! Defines the [Builder] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-25

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::fmt::{self,Debug,Display,Formatter};
use core::{hint,mem};
use crate::exprs::Expr;
use crate::exprs::expr_inners::ExprInner;
use crate::tokens::Token;
use self::builder_inners::BuilderInner::{self,*};

mod builder_inners;

/// Builder of [Exprs][Expr]
#[repr(transparent)]
pub struct Builder<Head, Alloc>(BuilderInner<Head,Alloc>)
  where Alloc: Allocator;

impl<Head, Alloc> Builder<Head, Alloc>
  where Alloc: Allocator {
  /// Constructs a builder with a hole.
  pub const fn hole() -> Self { Self(BHole) }
  /// Constructs a builder which represents [Expr].
  pub const fn from_expr(expr: Expr<Head, Alloc>) -> Self { Self(BExpr(expr)) }
  /// Constructs a builder which represents a token with no child expressions.
  ///
  /// # Params
  ///
  /// head_token --- Token text at the head of this expression.  
  /// allocator --- Allocator of the expression.  
  pub const fn from_token_in(head_token: Head, allocator: Alloc) -> Self
    where Head: Display {
    Self::from_expr(Expr::from_token_in(head_token,allocator))
  }
  /// Constructs a builder which represents a default expression.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the expression.  
  pub fn new_in(allocator: Alloc) -> Self
    where Head: Display + Default { Self::from_token_in(Default::default(),allocator) }
  /// Tests if this builder is a hole to be filled.
  pub const fn is_hole(&self) -> bool { if let BHole = self.0 { true } else { false } }
  /// Compares a partially built [Expr] against an [Expr].
  ///
  /// # Params
  ///
  /// expr --- [Expr] to compare with.  
  pub fn eq_expr<Head2,Alloc2>(&self, expr: &Expr<Head2, Alloc2>) -> bool
    where Head: PartialEq<Head2>, Alloc2: Allocator {
    match &self.0 {
      BHole      => false,
      BExpr(lhs) => lhs == expr,
      BPart(lhs) => lhs == expr,
    }
  }
  /// Gets the child expressions under construction.
  ///
  /// # Panics
  ///
  /// If `self` is a hole; use [is_hole][Self::is_hole] to test.
  pub fn child_exprs(&mut self) -> &mut Vec<Self, Alloc> {
    debug_assert!(!self.is_hole(),"can't reference child expressions of a hole");

    match &mut self.0 {
      BHole             => panic!("can't reference child expressions of a hole"),
      BPart(builder)    => &mut builder.child_exprs,
      //Deconstruct expression
      builder@BExpr(_)  => {
        let BExpr(expr) = mem::replace(builder,BHole)
          else { if cfg!(debug_assertions) { unreachable!("matched `builder` as `BExpr`") }
                 else { unsafe { hint::unreachable_unchecked() } } };
        let (head_token,child_exprs,fmt_expr) = expr.into_parts();
        let child_exprs = map_in_place::vec::alloc::map(child_exprs,Self::from_expr);

        *builder = BPart(ExprInner::from_parts(head_token,child_exprs,fmt_expr));
        if let BPart(builder) = builder { &mut builder.child_exprs }
        else { if cfg!(debug_assertions) { unreachable!("matched `builder` as `BExpr`") }
               else { unsafe { hint::unreachable_unchecked() } } }
      },
    }
  }
  /// Pushes a child expression.
  ///
  /// # Params
  ///
  /// child --- New child expression.  
  ///
  /// # Panics
  ///
  /// If `self` is a hole; use [is_hole][Self::is_hole] to test.
  pub fn push_child(&mut self, child: Self) -> &mut Self { self.child_exprs().push(child); self }
  /// Pushes a child expression.
  ///
  /// # Params
  ///
  /// child --- New child expression.  
  ///
  /// # Panics
  ///
  /// If `self` is a hole; use [is_hole][Self::is_hole] to test.
  pub fn push_expr(&mut self, child: Expr<Head, Alloc>) -> &mut Self {
    debug_assert!(!self.is_hole(),"can't reference child expressions of a hole");

    match &mut self.0 {
      BHole          => panic!("can't reference child expressions of a hole"),
      BExpr(expr)    => expr.child_exprs_mut().push(child),
      BPart(builder) => builder.child_exprs.push(Self::from_expr(child)),
    }

    self
  }
  /// Pushes a token as a child expression.
  ///
  /// # Params
  ///
  /// token --- Token constituting the new expression.  
  /// allocator --- Allocator of the new expression.  
  ///
  /// # Panics
  ///
  /// If `self` is a hole; use [is_hole][Self::is_hole] to test.
  pub fn push_token_in(&mut self, token: Head, allocator: Alloc) -> &mut Self
    where Head: Display {
    self.push_expr(Expr::from_token_in(token,allocator))
  }
  /// Tests that `self` contains no holes.
  pub const fn can_finish(&self) -> bool {
    match &self.0 {
      BHole          => false,
      BExpr(_expr)   => true,
      BPart(builder) => {
        let child_exprs = builder.child_exprs.as_slice();
        let mut index = 0;

        while index < child_exprs.len() {
          let child = &child_exprs[index];

          if !child.can_finish() { return false }
          index += 1;
        }

        true
      },
    }
  }
  /// Constructs an [Expr].
  ///
  /// # Panics
  ///
  /// If `self` contains holes; use [can_finish][Self::can_finish] to test.
  pub fn finish(self) -> Expr<Head, Alloc>
    where Alloc: Allocator {
    debug_assert!(self.can_finish(),"cant finish an expression with holes");

    match self.0 {
      BHole => panic!("cant finish an expression with holes"),
      BExpr(expr) => expr,
      BPart(builder) => {
        let (head_token,child_exprs,fmt_expr) = builder.into_parts();
        let child_exprs = map_in_place::vec::alloc::map(child_exprs,Self::finish);

        Expr::from_parts(head_token,child_exprs,fmt_expr)
      },
    }
  }
}

impl<Alloc> Builder<Token<Alloc>, Alloc>
  where Alloc: Allocator {
  /// Constructs a builder which represents a [Token] with no child expressions.
  ///
  /// # Params
  ///
  /// head_token --- [Token] at the head of this expression.  
  pub fn from_token(head_token: Token<Alloc>) -> Self
    where Alloc: Clone {
    let alloc = head_token.allocator().clone();

    Self::from_token_in(head_token,alloc)
  }
  /// Constructs a builder which represents a [Token] with no child expressions.
  ///
  /// # Params
  ///
  /// head_token --- Text at the head of this expression.  
  /// allocator --- Allocator of the expression.  
  pub fn from_str_in(token: &str, allocator: Alloc) -> Self
    where Alloc: Clone { Self::from_token(Token::from_str_in(token,allocator)) }
  /// Pushes a [Token] as a child expression.
  ///
  /// # Params
  ///
  /// token --- Token constituting the new expression.  
  ///
  /// # Panics
  ///
  /// If `self` is a hole; use [is_hole][Self::is_hole] to test.
  pub fn push_token(&mut self, token: Token<Alloc>) -> &mut Self
    where Alloc: Clone {
    let allocator = token.allocator().clone();

    self.push_token_in(token,allocator)
  }
  /// Pushes text as a child expression.
  ///
  /// # Params
  ///
  /// token --- Text constituting the new expression.  
  ///
  /// # Panics
  ///
  /// If `self` is a hole; use [is_hole][Self::is_hole] to test.
  pub fn push_str_in(&mut self, token: &str, allocator: Alloc) -> &mut Self
    where Alloc: Clone { self.push_token(Token::from_str_in(token,allocator)) }
}

impl<Head, Alloc> Clone for Builder<Head, Alloc>
  where Head: Clone, Alloc: Allocator + Clone {
  fn clone(&self) -> Self { Self(self.0.clone()) }
  fn clone_from(&mut self, source: &Self) { self.0.clone_from(&source.0) }
}

impl<Head1, Head2, Alloc1, Alloc2> PartialEq<BuilderInner<Head2,Alloc2>> for Builder<Head1, Alloc1>
  where Head1: PartialEq<Head2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &BuilderInner<Head2,Alloc2>) -> bool { self.0 == *rhs }
}

impl<Head1, Head2, Alloc1, Alloc2> PartialEq<Builder<Head2,Alloc2>> for Builder<Head1, Alloc1>
  where Head1: PartialEq<Head2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Builder<Head2,Alloc2>) -> bool { *self == rhs.0 }
}

impl<Head, Alloc> Debug for Builder<Head, Alloc>
  where Head: Debug, Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    match &self.0 {
      BHole          => write!(fmt,"Builder"),
      BExpr(expr)    => write!(fmt,"{:?}",expr),
      BPart(builder) => {
        write!(fmt,"Builder {{ ")?;
        builder.fmt_fields(fmt)?;
        write!(fmt," }}")
      },
    }
  }
}

impl<Head1, Alloc1> Expr<Head1, Alloc1>
  where Alloc1: Allocator {
  /// Compares an [Expr] against a partially built [Expr].
  pub fn eq_builder<Head2,Alloc2>(&self, builder: &Builder<Head2, Alloc2>) -> bool
    where Head1: PartialEq<Head2>, Alloc2: Allocator {
    match &builder.0 {
      BHole          => false,
      BExpr(expr)    => self == expr,
      BPart(builder) => self == builder,
    }
  }
}

impl<Head1, Head2, Alloc1, Alloc2> PartialEq<Expr<Head2,Alloc2>> for Builder<Head1, Alloc1>
  where Head1: PartialEq<Head2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Expr<Head2, Alloc2>) -> bool { self.eq_expr(rhs) }
}
