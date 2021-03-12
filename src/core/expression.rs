use dyn_clonable::*;
use std::{fmt, ops, collections::HashMap};
use crate::core;
use crate::core::Brackets;


#[clonable]
pub trait Expression: fmt::Display + Clone + fmt::Debug {
    fn args(&self) -> Option<Vec<Box<dyn Expression>>>;
    fn eval(&self) -> Option<f64>;
    fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64>;

    fn braced(self) -> Brackets;

    fn is_action(&self) -> bool { true }
    fn is_variable(&self) -> bool { false }
    fn is_value(&self) -> bool { false }
    fn is_function(&self) -> bool { false }
    fn own_priority(&self) -> u8 { 13 }
    fn priority(&self) -> u8 { 0 }
}