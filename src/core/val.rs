use crate::core::*;
use std::collections::HashMap;
use std::convert::{TryFrom, From };
use std::fmt;

pub type ValueCore = f64;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Value(pub ValueCore);

impl TryFrom<Value> for ValueCore
{
    type Error = ArithmeticalError;

    fn try_from(value: Value) -> Result<Self, Self::Error>
    {
        Ok(value.0)
    }
}

impl From<ValueCore> for Value
{
    fn from(value: ValueCore) -> Self
    {
        Value(value)
    }
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}


impl Expression for Value {}
impl Arithmetical for Value {}

impl Eval for Value
{
    fn eval_vars(&self, _variables: &HashMap<Variable, ValueCore>) -> EvalResult
    {
        ValueCore::try_from(*self)
    }
}

impl Priority for Value
{
    fn priority(&self) -> i8 { 1 }
}