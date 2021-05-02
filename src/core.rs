use std::fmt;

mod bin_operators;
mod var;
mod val;
mod expr;
mod eval;
pub use bin_operators::*;
pub use var::Variable;
pub use val::{ Value, ValueCore };
pub use expr::Expr;
pub use eval::{ Eval, ArithmeticalError, EvalResult };

pub const X: Variable = Variable("x");
pub const Y: Variable = Variable("y");
pub const ZERO: Value = Value(0.0);
pub const ONE: Value = Value(1.0);
pub const TWO: Value = Value(2.0);


pub trait Arithmetical: Expression
{
    fn add<E: Expression>(self, rhs: E) -> Add where Self: Sized
    {
        Add::new(self, rhs)
    }
    fn sub<E: Expression>(self, rhs: E) -> Sub where Self: Sized
    {
        Sub::new(self, rhs)
    }
    fn mul<E: Expression>(self, rhs: E) -> Mul where Self: Sized
    {
        Mul::new(self, rhs)
    }
    fn div<E: Expression>(self, rhs: E) -> Div where Self: Sized
    {
        Div::new(self, rhs)
    }
    fn pow<E: Expression>(self, rhs: E) -> Pow where Self: Sized
    {
        Pow::new(self, rhs)
    }
}


pub trait Expression: 'static + fmt::Display + fmt::Debug + Eval + Priority
{
    /// Wraps current expression with Expr
    fn get_expr(self) -> Expr where Self: Sized { Expr::new(self) }
}

pub trait Priority
{
    /// Priority of operation.
    /// See: https://ru.wikipedia.org/wiki/%D0%9F%D1%80%D0%B8%D0%BE%D1%80%D0%B8%D1%82%D0%B5%D1%82_%D0%BE%D0%BF%D0%B5%D1%80%D0%B0%D1%86%D0%B8%D0%B8
    ///
    /// The lower the number, the higher the priority.
    /// This is only necessary for visual placement of parentheses
    fn priority(&self) -> i8 { 13 }
}

#[cfg(test)]
mod tests
{
    use crate::core::*;
    use std::collections::HashMap;

    #[test]
    fn test_to_string()
    {
        let add = X.add(Y);
        assert_eq!(add.to_string(), "x + y");
        let div = X.div(Y);
        assert_eq!(div.to_string(), "x / y");
        let sub = X.sub(Y);
        assert_eq!(sub.to_string(), "x - y");
        let pow = X.pow(Y);
        assert_eq!(pow.to_string(), "x ^ y");
        let mul = X.mul(Y);
        assert_eq!(mul.to_string(), "x * y");
    }

    #[test]
    fn test_eval()
    {
        let val = Value(1.0);
        assert_eq!(val.eval().unwrap(), 1.0);
        let var = Variable("x");
        assert_eq!(var.eval(), EvalResult::Err(ArithmeticalError::UnknownVariable(var)));
        let add = TWO.add(TWO);
        assert_eq!(add.eval().unwrap(), 4.0);
        let mul = TWO.mul(TWO);
        assert_eq!(mul.eval().unwrap(), 4.0);
        let sub = TWO.sub(TWO);
        assert_eq!(sub.eval().unwrap(), 0.0);
        let div = TWO.div(TWO);
        assert_eq!(div.eval().unwrap(), 1.0);
        let div_zero = TWO.div(ZERO);
        assert_eq!(div_zero.eval(), EvalResult::Err(ArithmeticalError::ZeroDivision));
        let pow = TWO.pow(TWO);
        assert_eq!(pow.eval().unwrap(), 4.0)
    }

    #[test]
    fn test_eval_vars()
    {
        let vars =
            {
                let mut tmp = HashMap::new(); tmp.insert(X, 10.0); tmp
            };
        let err_eval = Y;
        assert_eq!(err_eval.eval_vars(&vars), EvalResult::Err(ArithmeticalError::UnknownVariable(Y)));
        let var = X;
        assert_eq!(var.eval_vars(&vars).unwrap(), 10.0);
        let add = X.add(X);
        assert_eq!(add.eval_vars(&vars).unwrap(), 20.0);
        let add_mul = X.add(X).mul(X);
        assert_eq!(add_mul.eval_vars(&vars).unwrap(), 200.0);
        let mul_add = X.mul(X).add(X);
        assert_eq!(mul_add.eval_vars(&vars).unwrap(), 110.0);
    }
}

