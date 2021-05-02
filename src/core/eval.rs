use crate::core::*;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ArithmeticalError
{
    /// If division by zero occurs
    ///
    /// ## Example:
    /// ```rust
    /// use rusymbols::core::*;
    ///
    /// let expr = TWO.div(ZERO);
    /// assert_eq!(expr.eval(), EvalResult::Err(ArithmeticalError::ZeroDivision))
    /// ```
    ZeroDivision,
    /// If the expression contains a variable for which no value was supplied,
    /// this error will be returned
    ///
    /// ## Example:
    /// ```rust
    /// use rusymbols::core::*;
    /// use std::collections::HashMap;
    ///
    /// let x = X;
    /// let x_add_y = X.add(Y);
    ///
    /// let vars =
    /// {
    ///     let mut tmp = HashMap::new();
    ///     tmp.insert(X, 10.0);
    ///     tmp
    /// };
    ///
    /// assert_eq!(x.eval_vars(&vars).unwrap(), 10.0);
    /// assert_eq!(x_add_y.eval_vars(&vars), EvalResult::Err(ArithmeticalError::UnknownVariable(Y)));
    /// ```
    UnknownVariable(Variable),
    Other(&'static str),
}

impl fmt::Display for ArithmeticalError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl Error for ArithmeticalError {}

pub type EvalResult = Result<ValueCore, ArithmeticalError>;

pub trait Eval
{
    /// Does the same as eval_vars, but does not provide any values
    fn eval(&self) -> EvalResult
    {
        self.eval_vars(&HashMap::new())
    }
    /// Tries to evaluate the value of an expression with the given variable values.
    /// ## Example:
    /// ```rust
    /// use rusymbols::core::*;
    /// use std::collections::HashMap;
    ///
    /// let x = X;
    /// let x_add_y = X.add(Y);
    ///
    /// let vars =
    /// {
    ///     let mut tmp = HashMap::new();
    ///     tmp.insert(X, 10.0);
    ///     tmp.insert(Y, -10.0);
    ///     tmp
    /// };
    ///
    /// assert_eq!(x.eval_vars(&vars).unwrap(), 10.0);
    /// assert_eq!(x_add_y.eval_vars(&vars).unwrap(), 0.0);
    ///
    /// ```
    fn eval_vars(&self, variables: &HashMap<Variable, ValueCore>) -> EvalResult;
}