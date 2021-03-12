use crate::core::expression::*;
use crate::core::actions::*;
use std::{fmt, ops, collections::HashMap};
use crate::core::Variable;

const ROUND: (&str, &str) = ("(", ")");

#[derive(Clone, Debug)]
pub struct Brackets {
    expression: Box<dyn Expression>,
    form: (&'static str, &'static str),
}

impl Brackets {
    pub fn new(expression: Box<dyn Expression>, form: (&'static str, &'static str)) -> Self {
        Self { expression, form }
    }

    pub fn new_round(expression: Box<dyn Expression>) -> Self {
        Self::new(expression, ROUND)
    }
}

impl Expression for Brackets {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>> { Some(vec![self.expression.clone()]) }
    fn eval(&self) -> Option<f64> { Some(self.expression.eval()?) }
    fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64> {
        Some(self.expression.eval_args(args)?)
    }

    fn braced(self) -> Brackets {
        Brackets::new_round(Box::new(self.clone()))
    }

    fn is_action(&self) -> bool { false }
}


impl fmt::Display for Brackets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.form.0)?;
        f.write_str(&self.expression.to_string())?;
        f.write_str(self.form.1)
    }
}
impl ops::Add for Brackets {
    type Output = Add;
    fn add(self, rhs: Self) -> Self::Output {
        Add::new(Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub for Brackets {
    type Output = Sub;

    fn sub(self, rhs: Self) -> Self::Output {
        Sub::new(Box::new(self), Box::new(rhs))
    }
}