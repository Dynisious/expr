//! Defines the [Builder] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-28

use alloc::alloc::Allocator;
use alloc::vec::Vec;
use core::fmt::{self,Debug,Display,Formatter};
use core::{hint,mem};
use crate::exprs::{self,Expr,ExprInner,FmtExpr};
use crate::tokens::Token;
use Builder::*;

/// Builder of [Exprs][Expr]
pub enum Builder<Token, Alloc>
  where Alloc: Allocator {
  /// Hole to be filled with an [Expr].
  BHole,
  /// Hole to be filled with a head `Token`.
  BTokenHole {
    /// Child expressions of this expression.
    child_exprs: Vec<Builder<Token,Alloc>,Alloc>,
    /// Formatting method for [Displaying][Display] [Exprs][Expr].
    fmt_expr: FmtExpr<Token,Alloc>,
  },
  /// Finished [Expr].
  BExpr(Expr<Token,Alloc>),
  /// [Expr] under construction.
  BPart(ExprInner<Token,Vec<Builder<Token,Alloc>,Alloc>,FmtExpr<Token,Alloc>>),
}

impl<Token, Alloc> Builder<Token, Alloc>
  where Alloc: Allocator {
  /// Constructs a builder which represents `token`.
  ///
  /// # Params
  ///
  /// head_token --- Token text at the head of this expression.  
  /// allocator --- Allocator of the expression.  
  pub const fn from_token_in(token: Token, allocator: Alloc) -> Self
    where Token: Display { BExpr(Expr::from_token_in(token,allocator)) }
  /// Constructs an empty expression.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the expression.  
  pub fn new_in(allocator: Alloc) -> Self
    where Token: Display {
    let child_exprs = Vec::new_in(allocator);
    let fmt_expr = exprs::fmt_expr;

    BTokenHole{child_exprs,fmt_expr}
  }
  /// Tests if this builder is a hole to be filled.
  pub const fn is_hole(&self) -> bool {
    match self {
      BHole | BTokenHole{..} => true,
      _                      => false,
    }
  }
  /// Tests if this builder has child expressions.
  pub const fn has_children(&self) -> bool {
    match self {
      BTokenHole{..} | BExpr(_) | BPart(_) => true,
      _                                    => false,
    }
  }
  /// Compares a partially built [Expr] against an [Expr].
  ///
  /// # Params
  ///
  /// expr --- [Expr] to compare with.  
  fn eq_expr<Token2,Alloc2>(&self, expr: &Expr<Token2, Alloc2>) -> bool
    where Token: PartialEq<Token2>, Alloc2: Allocator {
    match self {
      BHole | BTokenHole { .. } => false,
      BExpr(lhs)        => lhs == expr,
      BPart(lhs)        => lhs == expr,
    }
  }
  /// Takes the head `Token` of the [Expr].
  pub fn take_token(&mut self) -> Option<Token> {
    use map_in_place::vec::alloc;

    let (head_token,child_exprs,fmt_expr) = match &mut *self {
        BHole | BTokenHole{..} => return None,
        slot@BExpr(_) => {
          let BExpr(expr) = mem::replace(slot,BHole)
            else { if cfg!(debug_assertions) { unreachable!("matched `builder` as `BExpr`") }
                   else { unsafe { hint::unreachable_unchecked() } } };
          let (head_token,child_exprs,fmt_expr) = expr.into_parts();
          let child_exprs = alloc::map(child_exprs,BExpr);

          (head_token,child_exprs,fmt_expr)
        },
        slot@BPart(_) => {
          let BPart(builder) = mem::replace(slot,BHole)
            else { if cfg!(debug_assertions) { unreachable!("matched `builder` as `BPart`") }
                   else { unsafe { hint::unreachable_unchecked() } } };
          let (head_token,child_exprs,fmt_expr) = builder.into_parts();

          (head_token,child_exprs,fmt_expr)
        },
      };

    *self = BTokenHole{child_exprs,fmt_expr};
    Some(head_token)
  }
  /// Gets the child expressions under construction.
  ///
  /// # Panics
  ///
  /// If `self` does not have child expressions; use [has_children][Self::has_children] to test.
  pub fn child_exprs(&mut self) -> &mut Vec<Self, Alloc> {
    debug_assert!(self.has_children(),"can't reference child expressions of a hole");

    match self {
      BHole => panic!("can't reference child expressions of a hole"),
      BTokenHole{child_exprs,..} => child_exprs,
      BPart(builder)             => &mut builder.child_exprs,
      //Deconstruct expression
      builder@BExpr(_)           => {
        let BExpr(expr) = mem::replace(builder,BHole)
          else { if cfg!(debug_assertions) { unreachable!("matched `builder` as `BExpr`") }
                 else { unsafe { hint::unreachable_unchecked() } } };
        let (head_token,child_exprs,fmt_expr) = expr.into_parts();
        let child_exprs = map_in_place::vec::alloc::map(child_exprs,BExpr);

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
  /// If `self` does not have child expressions; use [has_children][Self::has_children] to test.
  pub fn push_child(&mut self, child: Self) -> &mut Self { self.child_exprs().push(child); self }
  /// Pushes a child expression.
  ///
  /// # Params
  ///
  /// child --- New child expression.  
  ///
  /// # Panics
  ///
  /// If `self` does not have child expressions; use [has_children][Self::has_children] to test.
  pub fn push_expr(&mut self, child: Expr<Token, Alloc>) -> &mut Self {
    debug_assert!(self.has_children(),"can't reference child expressions of a hole");

    let child_exprs = match self {
        BHole                      => panic!("can't reference child expressions of a hole"),
        BTokenHole{child_exprs,..} => child_exprs,
        BExpr(expr) => {
          expr.child_exprs.push(child);
          return self;
        },
        BPart(builder) => &mut builder.child_exprs,
      };

    child_exprs.push(BExpr(child));
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
  /// If `self` does not have child expressions; use [has_children][Self::has_children] to test.
  pub fn push_token_in(&mut self, token: Token, allocator: Alloc) -> &mut Self
    where Token: Display {
    self.push_expr(Expr::from_token_in(token,allocator))
  }
  /// Pushes an empty child expression.
  ///
  /// # Params
  ///
  /// allocator --- Allocator of the new expression.  
  pub fn push_alloc(&mut self, allocator: Alloc) -> &mut Self
    where Token: Display {
    self.push_child(Self::new_in(allocator))
  }
  /// Tests that `self` contains no holes.
  pub const fn can_finish(&self) -> bool {
    match self {
      BHole | BTokenHole{..} => false,
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
  pub fn finish(self) -> Expr<Token, Alloc>
    where Alloc: Allocator {
    debug_assert!(self.can_finish(),"cant finish an expression with holes");

    match self {
      BHole | BTokenHole{..} => panic!("cant finish an expression with holes"),
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

impl<Token, Alloc> Clone for Builder<Token, Alloc>
  where Token: Clone, Alloc: Allocator + Clone {
  fn clone(&self) -> Self {
    match self {
      BHole                            => BHole,
      BTokenHole{child_exprs,fmt_expr} => {
        let child_exprs = child_exprs.clone();
        let fmt_expr = fmt_expr.clone();
        
        BTokenHole{child_exprs,fmt_expr}
      },
      BExpr(expr)    => BExpr(expr.clone()),
      BPart(builder) => BPart(builder.clone()),
    }
  }
  fn clone_from(&mut self, source: &Self) {
    match (source,self) {
      (BExpr(expr),   BExpr(dest)) => dest.clone_from(expr),
      (BPart(builder),BPart(dest)) => dest.clone_from(builder),
      (source,        dest)        => *dest = source.clone(),
    }
  }
}

impl<Token1, Token2, Alloc1, Alloc2> PartialEq<Builder<Token2,Alloc2>> for Builder<Token1, Alloc1>
  where Token1: PartialEq<Token2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Builder<Token2,Alloc2>) -> bool {
    match (self,rhs) {
      (BHole,             _) | (_, BHole)             => false,
      (BTokenHole { .. }, _) | (_, BTokenHole { .. }) => false,
      (BExpr(lhs),     BExpr(rhs))     => lhs == rhs,
      (BExpr(expr),    BPart(builder)) => expr == builder,
      (BPart(builder), BExpr(expr))    => builder == expr,
      (BPart(lhs),     BPart(rhs))     => lhs == rhs,
    }
  }
}

impl<Token1, Token2, Alloc1, Alloc2> PartialEq<Expr<Token2,Alloc2>> for Builder<Token1, Alloc1>
  where Token1: PartialEq<Token2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Expr<Token2, Alloc2>) -> bool { self.eq_expr(rhs) }
}

impl<Token, Alloc> Debug for Builder<Token, Alloc>
  where Token: Debug, Alloc: Allocator {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    match self {
      BHole => write!(fmt,"Builder"),
      BTokenHole{child_exprs,fmt_expr} => {
        write!(fmt,"Builder {{ ")?;
        write!(fmt,"child_exprs: {:?}, fmt_expr: {:?}",child_exprs,fmt_expr)?;
        write!(fmt," }}")
      },
      BExpr(expr)    => write!(fmt,"{:?}",expr),
      BPart(builder) => {
        write!(fmt,"Builder {{ ")?;
        builder.fmt_fields(fmt)?;
        write!(fmt," }}")
      },
    }
  }
}

impl<Token1, Alloc1> Expr<Token1, Alloc1>
  where Alloc1: Allocator {
  /// Compares an [Expr] against a partially built [Expr].
  fn eq_builder<Token2,Alloc2>(&self, builder: &Builder<Token2, Alloc2>) -> bool
    where Token1: PartialEq<Token2>, Alloc2: Allocator {
    match builder {
      BHole | BTokenHole{..} => false,
      BExpr(expr)    => self == expr,
      BPart(builder) => self == builder,
    }
  }
}

impl<Token1, Token2, Alloc1, Alloc2> PartialEq<Builder<Token2,Alloc2>> for Expr<Token1, Alloc1>
  where Token1: PartialEq<Token2>, Alloc1: Allocator, Alloc2: Allocator {
  fn eq(&self, rhs: &Builder<Token2,Alloc2>) -> bool { self.eq_builder(rhs) }
}
