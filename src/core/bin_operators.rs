//! Two-operand operations such as multiplication and addition
//!
//! ## Examples:
//!
//! ```rust
//! use rusymbols::core::*;
//!
//! let add = Add::new(X, Y);
//! let add_from_method = X.add(Y);
//!
//! assert_eq!(add, add_from_method);
//!
//! // For the rest of the operations, the same is available.
//! ```

use crate::core::*;
use std::collections::HashMap;
use std::sync::Arc;

/// This macro generates code for binary operations.
/// For example Sub:
///
/// ## Example:
/// ```no_run
/// use rusymbols::core::*;
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use std::fmt;
/// const SUB_SIGN: &str = " - ";
///
/// #[derive(Debug, Clone)]
/// pub struct Sub(pub Arc<dyn Expression>, pub Arc<dyn Expression>);
///
/// impl Expression for Sub {}
/// impl Arithmetical for Sub {}
///
/// impl Sub
/// {
///     pub fn new<L: Expression, R: Expression>(left: L, right: R) -> Self
///     {
///         Self(Arc::new(left), Arc::new(right))
///     }
/// }
///
/// impl fmt::Display for Sub
/// {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
///     {
///             let (left, middle, right) = (self.0.priority(), self.priority(), self.1.priority());
///             f.write_str((if left >= middle { format!("({})", self.0) } else { format!("{}", self.0) }).as_str())?;
///             f.write_str(SUB_SIGN)?;
///             f.write_str((if right >= middle { format!("({})", self.1) } else { format!("{}", self.1) }).as_str())
///     }
/// }
///
/// impl Eval for Sub
/// {
///     fn eval_vars(&self, variables: &HashMap<Variable, ValueCore>) -> EvalResult
///     {
///         let left = self.0.eval_vars(variables)?;
///         let right = self.1.eval_vars(variables)?;
///         Ok(left - right)
///     }
/// }
///
/// impl Priority for Sub
/// {
///     fn priority(&self) -> i8 { 5 }
/// ```
macro_rules! bin_operator {
    ($operator:ident, sign=$sign:literal, eval=$eval:ident, priority=$priority:literal) =>
    {
    impl Expression for $operator {}
    impl Arithmetical for $operator {}

    impl $operator
    {
        /// Automatically places in an Arc container and creates an instance of operation
        pub fn new<L: Expression, R: Expression>(left: L, right: R) -> Self
        {
            Self(Arc::new(left), Arc::new(right))
        }
    }

    impl fmt::Display for $operator
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            // needed to place parentheses to show the priority of operations
            let (left, middle, right) = (self.0.priority(), self.priority(), self.1.priority());
            f.write_str((if left >= middle { format!("({})", self.0) } else { format!("{}", self.0) }).as_str())?;
            f.write_str($sign)?;
            f.write_str((if right >= middle { format!("({})", self.1) } else { format!("{}", self.1) }).as_str())

        }
    }
    impl Eval for $operator
    {
        fn eval_vars(&self, variables: &HashMap<Variable, ValueCore>) -> EvalResult
        {
        let left = self.0.eval_vars(variables)?;
        let right = self.1.eval_vars(variables)?;
        $eval(left, right)
        }
    }
    impl Priority for $operator
    {
        fn priority(&self) -> i8 { $priority }
    }

    }
}


/// Standard division
#[derive(Debug, Clone)]
pub struct Div(pub Arc<dyn Expression>, pub Arc<dyn Expression>);

/// Standard multiplication
#[derive(Debug, Clone)]
pub struct Mul(pub Arc<dyn Expression>, pub Arc<dyn Expression>);

/// Standard subtraction
#[derive(Debug, Clone)]
pub struct Sub(pub Arc<dyn Expression>, pub Arc<dyn Expression>);

/// Standard addition
#[derive(Debug, Clone)]
pub struct Add(pub Arc<dyn Expression>, pub Arc<dyn Expression>);

/// Standard power (rational)
#[derive(Debug, Clone)]
pub struct Pow(pub Arc<dyn Expression>, pub Arc<dyn Expression>);

#[inline]
fn eval_div(left: ValueCore, right: ValueCore) -> EvalResult
{
    if right != 0.0 { Ok(left / right) } else { Err(ArithmeticalError::ZeroDivision) }
}

#[inline]
fn eval_mul(left: ValueCore, right: ValueCore) -> EvalResult
{
    Ok(left * right)
}

#[inline]
fn eval_add(left: ValueCore, right: ValueCore) -> EvalResult
{
    Ok(left + right)
}

#[inline]
fn eval_sub(left: ValueCore, right: ValueCore) -> EvalResult
{
    Ok(left - right)
}

#[inline]
fn eval_pow(left: ValueCore, right: ValueCore) -> EvalResult
{
    Ok(left.powf(right))
}

bin_operator!(Pow, sign=" ^ ", eval=eval_pow, priority=3);
bin_operator!(Div, sign=" / ", eval=eval_div, priority=4);
bin_operator!(Mul, sign=" * ", eval=eval_mul, priority=4);
bin_operator!(Sub, sign=" - ", eval=eval_sub, priority=5);
bin_operator!(Add, sign=" + ", eval=eval_add, priority=5);
