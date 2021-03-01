use std::{ops, fmt, collections::HashMap};
use crate::core::actions::*;
use crate::core::expression::*;

#[derive(Clone, Debug)]
pub struct Variable {
    literal: String
}


impl Expression for Variable {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>> { None }
    fn eval(&self) -> Option<f64> { None }
    fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64> {
        Some(args.get(self.literal.as_str())?.clone())
    }
    fn is_action(&self) -> bool { false }
    fn is_variable(&self) -> bool { true }
    fn priority(&self) -> u8 { 1 }
}


impl Variable {
    pub fn new(literal: &str) -> Self { Self { literal: String::from(literal) } }
}


impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.literal)
    }
}

impl ops::Add for Variable {
    type Output = Add;
    fn add(self, rhs: Self) -> Self::Output {
        Add::new(Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub for Variable {
    type Output = Sub;
    fn sub(self, rhs: Self) -> Self::Output { Sub::new(Box::new(self), Box::new(rhs)) }
}


impl ops::Mul for Variable {
    type Output = Mul;
    fn mul(self, rhs: Self) -> Self::Output { Mul::new(Box::new(self), Box::new(rhs)) }
}
