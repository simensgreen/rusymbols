pub mod core;
pub use crate::core::Expression;

#[cfg(test)]
mod tests {
    use crate::core;
    use std::collections::HashMap;

    #[test]
    fn expressions_priority_eq() {
        assert_eq!(core::Actions::Add, core::Actions::Add);
        assert_eq!(core::Actions::Mul, core::Actions::Mul);
        assert_eq!(core::Actions::Div, core::Actions::Div);
        assert_eq!(core::Actions::Pow, core::Actions::Pow);
        assert_eq!(core::Actions::Brackets(Box::from(core::Expression::default()),
                                           core::Brackets::Round),
                   core::Actions::Brackets(Box::from(core::Expression::default()),
                                           core::Brackets::Round));
        assert_eq!(core::Actions::Var(String::from("x")), core::Actions::Var(String::from("x")));
        assert_eq!(core::Actions::Val(0.0), core::Actions::Val(1.0));
        assert_eq!(core::Actions::Val(2.0), core::Actions::Var(String::from("x")));
        assert_eq!(core::Actions::Var(String::from("x")), core::Actions::Val(3.0));
        assert_ne!(core::Actions::Mul, core::Actions::Add);
        assert_ne!(core::Actions::Add, core::Actions::Mul);
        assert_ne!(core::Actions::Add, core::Actions::Pow);
        assert_ne!(core::Actions::Mul, core::Actions::Pow);
    }

    #[test]
    fn expressions_priority_ord() {
        assert!(core::Actions::Add < core::Actions::Mul);
        assert!(core::Actions::Mul < core::Actions::Pow);
        assert!(core::Actions::Pow < core::Actions::Brackets(
            Box::new(core::Expression::default()), core::Brackets::Round))
    }


    #[test]
    fn addition() {
        let mut vars: HashMap<&str, f64> = HashMap::new();
        vars.insert("x", 10.0);
        vars.insert("y", 100.0);
        let x = core::Expression::new_var("x");
        let y = core::Expression::new_var("y");
        // let mut expr = x.clone().pow(y.clone());
        let expr = (x.clone() + y.clone()) * x;
        // let mul = expr.pow(expr2);
        println!("{}", expr.eval(&vars).unwrap())
    }
}
