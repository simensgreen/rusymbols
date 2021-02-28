use crate::core::*;
use std::fmt;
use std::cmp::Ordering;


const PRIORITY: [Actions; 5] = [
    Actions::Sub,
    Actions::Add,
    Actions::Mul,
    Actions::Pow,
    Actions::Div,
];



/// # Available math actions and service information.
///
/// These "actions" can be compared with each other to determine priority.
/// The higher the priority, the earlier the value will be calculated.
#[derive(Debug, Clone)]
pub enum Actions {
    /// subtraction "`-`"
    Sub,
    /// addition "`+`"
    Add,
    /// multiplication "`*`"
    Mul,
    /// division "`/`"
    Div,
    /// exponentiation "`**`"
    Pow,
    /// Not really an action - it's a variable literal
    Var(String),
    /// It's also not really an action - it's just a value or a number.
    Val(f64),
    /// Expression in parentheses. Brackets also shows the shape of the brackets.
    Brackets(Box<Expression>, Brackets),
}

impl Default for Actions {
    fn default() -> Self {
        Actions::Add
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Actions::Add => f.write_str(literals::SYMBOL_ADD),
            Actions::Mul => f.write_str(literals::SYMBOL_MUL),
            Actions::Pow => f.write_str(literals::SYMBOL_POW),
            Actions::Div => f.write_str(literals::SYMBOL_DIV),
            Actions::Sub => f.write_str(literals::SYMBOL_SUB),
            Actions::Var(literal) => f.write_str(&literal),
            Actions::Val(value) => f.write_str(&value.to_string()),
            Actions::Brackets(expr, brackets) => {
                let (left, right) = brackets.get_symbols();
                f.write_str(left)?;
                f.write_str(&expr.to_string())?;
                f.write_str(right)
            },
        }
    }
}

/// These "actions" can be compared with each other to determine priority.
/// The higher the priority, the earlier the value will be calculated.
///
/// # Examples
/// ```edition2018
/// use rusymbols::core::Actions;
/// assert_eq!(Actions::Add, Actions::Sub); // Subtraction and addition have equal priority.
/// assert_ne!(Actions::Add, Actions::Mul); // Addition and multiplication have different priorities.
/// ```
impl PartialEq for Actions {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Actions::Sub, Actions::Sub) => true,
            (Actions::Add, Actions::Add) => true,
            (Actions::Sub, Actions::Add) => true,
            (Actions::Add, Actions::Sub) => true,

            (Actions::Mul, Actions::Mul) => true,
            (Actions::Div, Actions::Div) => true,
            (Actions::Div, Actions::Mul) => true,
            (Actions::Mul, Actions::Div) => true,

            (Actions::Pow, Actions::Pow) => true,
            (Actions::Val(..), Actions::Val(..)) => true,
            (Actions::Var(..), Actions::Var(..)) => true,
            (Actions::Val(..), Actions::Var(..)) => true,
            (Actions::Var(..), Actions::Val(..)) => true,
            (Actions::Brackets(..), Actions::Brackets(..)) => true,
            _ => false
        }
    }
}


/// These "actions" can be compared with each other to determine priority.
/// The higher the priority, the earlier the value will be calculated.
///
///#Example:
/// ```edition2018
/// use rusymbols::core::Actions;
/// assert!(Actions::Add < Actions::Mul); // Subtraction and addition have equal priority.
/// assert!(Actions::Mul < Actions::Pow); // Addition and multiplication have different priorities.
/// ```
impl PartialOrd for Actions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut priority = Vec::from(PRIORITY);
        priority.push(Actions::Brackets(Box::from(Expression::default()), Brackets::Round));

        let self_prior = priority.iter().position(|x| x == self);
        let other_prior = priority.iter().position(|x| x == other);

        if self_prior.is_some() && other_prior.is_some() {
            let (self_prior, other_prior) = (self_prior.unwrap(), other_prior.unwrap());
            self_prior.partial_cmp(&other_prior)
        }
        else { return None }
    }
}