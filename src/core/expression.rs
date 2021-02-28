use std::fmt;
use std::ops;
use std::collections::HashMap;

use crate::core;
use crate::core::Actions;

/// # `Expression` struct
///
/// Probably the most important structure of this crate. It provides a tree-like structure
/// for expressions. The basic idea is that the `Expression` contains two arguments and the
/// action to be done with them. But so far, there are situations when there cannot be exactly two
/// arguments (for example, an ordinary number or a variable), so `Actions` have
/// taken responsibility for such cases.
///
/// # Example
/// ```edition2018
/// use rusymbols::Expression;
/// let x = Expression::new_var("x"); // new 'x' variable creation
/// let y = Expression::new_var("y"); // new 'y' variable creation
/// let expr = x + y; // expression
///
/// assert_eq!(expr.to_string(), "x + y")
/// ```
///
/// ## Beware of automatic parentheses
/// ```edition2018
/// use rusymbols::Expression;
/// let x = Expression::new_var("x"); // new 'x' variable creation
/// let y = Expression::new_var("y"); // new 'y' variable creation
/// let z = Expression::new_var("z"); // new 'z' variable creation
/// // according to the rules of arithmetic, multiplication takes precedence over addition
/// let expr = x.clone() + y.clone() * z.clone(); // equivalent to x + (y * z)
///
/// let mut expr_2 = x.clone() + y.clone(); // equivalent to x + y
/// expr_2 = expr_2 * z.clone(); // equivalent to (x + y) * z
///
/// let expr_3 = (x + y) * z;
///
/// assert_ne!(expr.to_string(), expr_2.to_string());
/// assert_eq!(expr_2.to_string(), expr_3.to_string());
///
/// assert_eq!(expr.to_string(), "x + y * z");
/// assert_eq!(expr_2.to_string(), "(x + y) * z");
/// ```
#[derive(Clone, Debug, Default)]
pub struct Expression {
    args: Vec<Expression>,
    kind: Actions,
}

/// Associated functions
impl Expression {
    /// New `Expression` constructor
    /// #### Example
    /// ```
    /// use rusymbols::{core, Expression};
    /// let x = Expression::new_var("x"); // new 'x' variable creation
    /// let two = Expression::new_val(2.0); // new 2.0 value creation
    /// let expr = Expression::new(x, two, core::Actions::Mul); // new expression creation
    ///
    /// assert_eq!(expr.to_string(), "x * 2")
    /// ```
    pub fn new(left: Expression, right: Expression, kind: Actions) -> Expression {
        Expression{ args: vec![left, right], kind}
    }

    /// New `Expression` constructor with variable literal
    /// #### Example
    /// ```
    /// use rusymbols::{core, Expression};
    /// let x = Expression::new_var("x"); // new 'x' variable creation
    /// let y = Expression::new_var("y"); // new 'y' value creation
    /// let expr = x / y;
    ///
    /// assert_eq!(expr.to_string(), "x / y")
    /// ```
    pub fn new_var(literal: &str) -> Expression {
        Expression { args: vec![], kind: Actions::Var(String::from(literal)) }
    }


    /// New `Expression` constructor with numeric value
    /// #### Example
    /// ```
    /// use rusymbols::{core, Expression};
    /// use std::collections::HashMap;
    ///
    /// let ten = Expression::new_val(10.0); // new 'x' variable creation
    /// let two = Expression::new_val(2.0); // new 2.0 value creation
    /// let expr = ten * two; // new expression creation
    /// let args: HashMap<&str, f64> = HashMap::new();
    ///
    /// assert_eq!(expr.to_string(), "10 * 2");
    /// assert_eq!(expr.eval_args(&args).unwrap(), 20.0);
    ///
    /// ```
    pub fn new_val(value: f64) -> Expression {
        Expression { args: vec![], kind: Actions::Val(value) }
    }

    ///New `Expression` in brackets
    pub fn new_brackets(expression: Expression, brackets: core::Brackets) -> Expression {
        Expression { args: vec![], kind: Actions::Brackets(Box::new(expression), brackets)}
    }
}


///Taking ownership of data
impl Expression {

    /// Wraps the given `Expression` with the specified parentheses
    /// #### Example
    /// ```
    /// use rusymbols::Expression;
    /// use rusymbols::core::Brackets;
    /// let x = Expression::new_var("x");
    /// let expr = x.brackets(Brackets::Square);
    ///
    /// assert_eq!(expr.to_string(), "[x]")
    /// ```
    #[inline]
    pub fn brackets(self, brackets: core::Brackets) -> Expression {
        Expression::new_brackets(self, brackets)
    }

    /// Wraps the given `Expression` with the round brackets
    /// #### Example
    /// ```
    /// use rusymbols::Expression;
    /// let x = Expression::new_var("x");
    /// let expr = x.brackets_round();
    ///
    /// assert_eq!(expr.to_string(), "(x)")
    /// ```
    #[inline]
    pub fn brackets_round(self) -> Expression {
        self.brackets(core::Brackets::Round)
    }

    /// Raises an `Expression` to a specified power
    /// #### Example
    /// ```
    /// use rusymbols::Expression;
    /// let x = Expression::new_var("x");
    /// let y = Expression::new_var("y");
    /// let expr = x.pow(y);
    ///
    /// assert_eq!(expr.to_string(), "x**y")
    /// ```
    pub fn pow(mut self, mut rhs: Self) -> Self {
        if self.kind < Actions::Pow { self = self.brackets_round() };
        if rhs.kind < Actions::Pow { rhs = rhs.brackets_round() };

        Expression::new(self, rhs, core::Actions::Pow)
    }

    /// Wraps the given `Expression` with parentheses if the given `Actions` has a higher priority
    #[inline(always)]
    fn brace_if(self, target: Actions) -> Self {
        if self.kind < target { self.brackets_round() }
        else { self }
    }
}

///Borrowing methods
impl Expression {
    /// Tries to get the numeric value of an `Expression` by replacing
    /// variables with values from a `HashMap`
    ///
    /// Returns None if the expression cannot be evaluated (e.g. insufficient variable values were passed)
    ///
    /// # Usage
    /// ```edition2018
    /// const LITERAL_X: &str = "x";
    /// const LITERAL_Y: &str = "y";
    ///
    /// use std::collections::HashMap;
    /// use rusymbols::Expression;
    ///
    /// let mut args: HashMap<&str, f64> = HashMap::new();
    /// args.insert(LITERAL_X, 2.0);
    /// args.insert(LITERAL_Y, 2.0);
    ///
    /// let x = Expression::new_var(LITERAL_X);
    /// let y = Expression::new_var(LITERAL_Y);
    /// let expr = x * y;
    ///
    /// assert_eq!(expr.eval_args(&args).unwrap(), 4.0);
    /// ```
    pub fn eval_args(&self, args: &HashMap<&str, f64>) -> Option<f64> {
        match &self.kind {
            Actions::Val(value) => Some(value.clone()),
            Actions::Var(var) => Some(args.get(var.as_str())?.clone()),
            Actions::Add => Some(self.args[0].eval_args(args)? + self.args[1].eval_args(args)?),
            Actions::Mul => Some(self.args[0].eval_args(args)? * self.args[1].eval_args(args)?),
            Actions::Pow => Some(self.args[0].eval_args(args)?.powf(self.args[1].eval_args(args)?)),
            Actions::Brackets(expr, _) => expr.eval_args(args),
            Actions::Div => {
                let denominator = Expression::new(self.args[1].clone(),
                                                  Expression::new_val(-1.0),
                                                  Actions::Pow);
                Expression::new(self.args[0].clone(),
                                denominator,
                                Actions::Mul).eval_args(args)
            },
            Actions::Sub => {
                Expression::new(self.args[0].clone(),
                                -self.args[1].clone(),
                                Actions::Add).eval_args(args)
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.kind.clone() {
            Actions::Val(value) => f.write_str(&value.to_string()),
            Actions::Var(literal) => f.write_str(&literal),
            Actions::Brackets(expr, brackets) => {
                let (left, right) = brackets.get_symbols();
                f.write_str(left)?;
                f.write_str(&expr.to_string())?;
                f.write_str(right)
            },
            _ => {
                f.write_str(&self.args[0].to_string())?;
                f.write_str(&self.kind.to_string())?;
                f.write_str(&self.args[1].to_string())
            }
        }
    }
}

impl ops::Add for Expression {
    type Output = Expression;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Actions::Add);
        rhs = rhs.brace_if(Actions::Add);
        Expression::new(self, rhs, Actions::Add)
    }
}

impl ops::Mul for Expression {
    type Output = Expression;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Actions::Mul);
        rhs = rhs.brace_if(Actions::Mul);
        Expression::new(self, rhs, core::Actions::Mul)
    }
}

impl ops::Div for Expression {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Actions::Div);
        rhs = rhs.brace_if(Actions::Div);
        Expression::new(self, rhs, Actions::Div)
    }
}

impl ops::Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        Expression::new(self, Expression::new_val(-1.0), Actions::Mul)
    }
}

impl ops::Sub for Expression {
    type Output = Expression;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Actions::Add);
        rhs = rhs.brace_if(Actions::Add);
        Expression::new(self, rhs, Actions::Sub)
    }
}