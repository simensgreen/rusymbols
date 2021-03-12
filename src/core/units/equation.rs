use crate::core::expression::*;
use crate::core::actions::*;
use std::{fmt, ops, collections::HashMap};
use crate::core::Brackets;

const LITERAL: &str = " = ";

#[derive(Clone, Debug)]
pub struct Equation {
    args: [Box<dyn Expression>; 2],
}

impl Equation {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { args: [left, right] }
    }
}

impl Expression for Equation {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>> { Some(Vec::from(self.args.clone())) }
    fn eval(&self) -> Option<f64> { Some(self.args[0].eval()? + self.args[1].eval()?) }
    fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64> {
        Some(self.args[0].eval_args(args)? + self.args[1].eval_args(args)?)
    }

    fn braced(self) -> Brackets {
        Brackets::new_round(Box::new(self.clone()))
    }

    fn is_action(&self) -> bool { true }
    fn own_priority(&self) -> u8 { 5 }
}


impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.args[0].to_string())?;
        f.write_str(LITERAL)?;
        f.write_str(&self.args[1].to_string())
    }
}