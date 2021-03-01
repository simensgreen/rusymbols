use dyn_clonable::*;
use std::{fmt, collections::HashMap};

pub(crate) const BRACKETS_ROUND: (&str, &str) = ("(", ")");


#[clonable]
pub trait Expression: fmt::Display + Clone + fmt::Debug {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>>;
    fn eval(&self) -> Option<f64>;
    fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64>;
    fn brace(&mut self) {}
    fn brace_form(&mut self, brackets: (&'static str, &'static str)) {}

    fn brackets(&self) -> Option<(&'static str, &'static str)> { None }
    fn is_action(&self) -> bool { true }
    fn is_variable(&self) -> bool { false }
    fn is_value(&self) -> bool { false }
    fn is_function(&self) -> bool { false }
    fn own_priority(&self) -> u8 { 13 }
    fn priority(&self) -> u8 { if self.brackets().is_none() { self.own_priority() } else { 1 } }
}
