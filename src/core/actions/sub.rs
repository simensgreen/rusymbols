use crate::core::expression::*;
use crate::core::actions::*;
use std::{ops, fmt, collections::HashMap};

const LITERAL: &str = " - ";

#[derive(Clone, Debug)]
pub struct Sub {
    args: [Box<dyn Expression>; 2],
    pub brackets: Option<(&'static str, &'static str)>
}

impl Expression for Sub {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>> { Some(Vec::from(self.args.clone())) }
    fn eval(&self) -> Option<f64> { Some(self.args[0].eval()? - self.args[1].eval()?) }
    fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64> {
        Some(self.args[0].eval_args(args)? - self.args[1].eval_args(args)?)
    }
    fn brace(&mut self) { self.brackets = Some(BRACKETS_ROUND) }
    fn brace_form(&mut self, brackets: (&'static str, &'static str)) { self.brackets = Some(brackets) }
    fn brackets(&self) -> Option<(&'static str, &'static str)> { self.brackets }
    fn is_action(&self) -> bool { true }
    fn own_priority(&self) -> u8 { 5 }
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.brackets.is_some() { f.write_str(self.brackets.unwrap().0)? };
        f.write_str(&self.args[0].to_string())?;
        f.write_str(LITERAL)?;
        if self.brackets.is_some() {
            f.write_str(&self.args[1].to_string())?;
            f.write_str(self.brackets.unwrap().1)
        } else {
            f.write_str(&self.args[1].to_string())
        }
    }
}

impl Sub {
    pub fn new(mut left: Box<dyn Expression>, mut right: Box<dyn Expression>) -> Self {
        if left.priority() > right.priority() { right.brace() }
        else if left.priority() < right.priority() { right.brace() }
        Self { args: [left, right], brackets: None }
    }
}


impl ops::Add for Sub {
    type Output = Add;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        if self.priority() > rhs.priority() { rhs.brace() }
        else if self.priority() < rhs.priority() { self.brace() }
        Add::new(Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub for Sub {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        if self.priority() > rhs.priority() { rhs.brace() }
        else if self.priority() < rhs.priority() { self.brace() }
        Sub::new(Box::new(self), Box::new(rhs))
    }
}
