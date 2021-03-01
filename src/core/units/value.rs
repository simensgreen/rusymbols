use std::{ops, fmt, collections::HashMap};
use crate::core::actions::*;
use crate::core::expression::*;

#[derive(Clone, Debug)]
pub struct Value {
    value: f64
}

impl Value {
    pub fn new(value: f64) -> Self { Self { value } }
}

impl Expression for Value {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>> { None }
    fn eval(&self) -> Option<f64> { Some(self.value.clone()) }
    fn eval_args(&self, _args: &HashMap<&str, f64>) -> Option<f64> { self.eval() }
    fn is_action(&self) -> bool { false }
    fn is_value(&self) -> bool { true }
    fn priority(&self) -> u8 { 1 }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.value.to_string())
    }
}

impl ops::Add for Value {
    type Output = Add;
    fn add(self, rhs: Self) -> Self::Output { Add::new(Box::new(self), Box::new(rhs)) }
}

impl ops::Sub for Value {
    type Output = Sub;
    fn sub(self, rhs: Self) -> Self::Output { Sub::new(Box::new(self), Box::new(rhs)) }
}

impl ops::Mul for Value {
    type Output = Mul;
    fn mul(self, rhs: Self) -> Self::Output { Mul::new(Box::new(self), Box::new(rhs)) }
}
