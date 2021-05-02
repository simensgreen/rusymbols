use crate::core::*;
use std::sync::Arc;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
/// A wrapper for any expression. Behaves in exactly the same way as the expression enclosed in it.
/// May be useful for purposes where type checking conflicts need to be resolved or the result is
/// unknown, for example, the result of an expression parser
///
/// Pattern `Facade` (maybe)
///
/// ## Example
/// ```rust
/// use rusymbols::core::*;
///
/// let pow = TWO.pow(TWO);
/// // assert_eq!(calc(pow), 4.0); // type checker worries
/// assert_eq!(calc(pow.clone().get_expr()), 4.0); // ok
///
/// assert_eq!(pow, 4.0); // ok, but in some cases it may not work
///
/// fn calc(expr: Expr) -> f64
/// {
///     expr.eval().unwrap()
/// }
///
/// fn generic_calc<E: Expression>(expr: E) -> f64
/// {
///     expr.eval().unwrap()
/// }
///
/// ```
pub struct Expr(Arc<dyn Expression>);

impl Expr
{
    pub fn new<E: Expression>(expression: E) -> Self
    {
        Self(Arc::new(expression))
    }
}

impl Expression for Expr {}

impl Arithmetical for Expr
{
    fn add<T: Expression>(self, rhs: T) -> Add
    {
        Add::new(self, rhs)
    }
}

impl fmt::Display for Expr
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}

impl Eval for Expr
{
    fn eval_vars(&self, variables: &HashMap<Variable, ValueCore>) -> EvalResult
    {
        self.0.eval_vars(variables)
    }
}

impl Priority for Expr
{
    fn priority(&self) -> i8 { self.0.priority() }
}
