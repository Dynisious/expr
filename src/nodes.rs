//! Defines the [Node] type.
//!
//! Author --- DMorgan  
//! Last Modified --- 2024-12-17

use core::fmt::{self,Display,Formatter};
#[cfg(doc)]
use crate::exprs::Expr;

pub mod impls;

/// The default formatting method for [Expr] values.
///
/// # Examples
///
/// ```
/// # #![feature(allocator_api)]
/// # extern crate alloc;
/// #
/// # use alloc::alloc::Global;
/// # use expr::exprs::Expr;
/// #
/// # let alloc = Global;
/// let expr_a = Expr::from_str("a",alloc);
///
/// assert_eq!("a",format!("{}",expr_a));
/// ```
///
/// ```
/// # #![feature(allocator_api)]
/// # extern crate alloc;
/// #
/// # use alloc::alloc::Global;
/// # use expr::exprs::Expr;
/// #
/// # let alloc = Global;
/// let mut expr_a = Expr::from_str("a",alloc);
/// let expr_b = Expr::from_str("b",alloc);
/// expr_a.push_expr(expr_b);
///
/// assert_eq!("a [b]",format!("{}",expr_a));
/// ```
///
/// # Params
///
/// head_token --- Head token of the expression to format.  
/// children --- Children expressions to format.  
/// fmt --- [Formatter] to write to.  
pub fn fmt_node<HeadToken,Iter>(head_token: &HeadToken, mut children: Iter,
                                       fmt: &mut Formatter) -> fmt::Result
  where HeadToken: Display, Iter: Iterator, <Iter as Iterator>::Item: Display {
  write!(fmt,"{}",head_token)?;

  if let Some(child_expr) = children.next() {
    write!(fmt," [{}",child_expr)?;
    for child_expr in children {
      write!(fmt,", {}",child_expr)?;
    }
    write!(fmt,"]")?;
  }

  Ok(())
}

/// Expression tree of [Tokens][impls::Impl::Token].
pub(crate) struct Node<Impl>
  where Impl: impls::Impl {
  /// Symbol at the head of this expression.
  pub head_token: Impl::Token,
  /// Child expressions.
  pub children: Impl::Children,
  /// A customisable formatting method for [Display].
  pub fmt: Impl::Fmt,
}

impl<Impl> Node<Impl>
  where Impl: impls::Impl {
  /// Deconstructs into parts.
  pub fn into_inner(self) -> (Impl::Token,Impl::Children,Impl::Fmt) {
    let Self { head_token, children, fmt } = self;

    (head_token,children,fmt)
  }
  /// Constructs a new expression from parts.
  ///
  /// # Params
  ///
  /// head_token --- Symbol at the head of this expression.  
  /// children --- Child expressions.  
  /// fmt --- A customisable formatting method for [Display].  
  pub const fn from_parts(head_token: Impl::Token, children: Impl::Children, fmt: Impl::Fmt) -> Self {
    Self { head_token, children, fmt }
  }
}

impl<Impl> Copy for Node<Impl>
  where Self: Clone, Impl: impls::Impl, Impl::Token: Copy, Impl::Children: Copy, Impl::Fmt: Copy {}
