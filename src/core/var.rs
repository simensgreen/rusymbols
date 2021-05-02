use crate::core::*;
use std::fmt;
use std::collections::HashMap;


/// Variable
///
/// ## Example:
/// ```rust
/// use rusymbols::core::*;
///
/// let literal = "x";
/// let x = Variable(literal);
/// let y = Variable("y");
/// let add = x.add(y);
///
/// assert_eq!(x.to_string(), "x");
/// assert_eq!(y.to_string(), "y");
/// assert_eq!(add.to_string(), "x + y");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Variable(pub &'static str);

impl Expression for Variable {}
impl Arithmetical for Variable {}

impl fmt::Display for Variable
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.write_str(self.0)
    }
}

impl Eval for Variable
{
    fn eval_vars(&self, variables: &HashMap<Variable, ValueCore>) -> EvalResult
    {
        if let Some(value) = variables.get(&self)
        {
            Ok(*value)
        }
        else
        {
            Err(ArithmeticalError::UnknownVariable(*self))
        }
    }
}

impl Priority for Variable
{
    fn priority(&self) -> i8 { 1 }
}