//! # About
//!
//! rusymbols is a Rust crate for symbolic mathematics. It aims to become a full-featured computer
//! algebra system (CAS) while keeping the code as simple as possible in order to be comprehensible
//! and easily extensible. rusymbols is written entirely in Rust.
//!
//! (maybe some engineers want to learn Rust :))
//!
//! The crate is designed to support the Rust language and add the ability to work with complex
//! formulas at the speed of Rust, possibly slower than programmed without a high level of
//! abstraction. Do not think about their solution, especially since there are formulas that change
//! depending on the circumstances.The goal of the project is at least to become similar to SymPy.
//!
//! # Key ideas:
//!     - Simple.
//!     - Fast (maybe not at maximum speed, but still fast)
//!     - Safe
//!     - Universal (compatibility with nalgebra and standard types (possibly) and other similar crates)
//!
//! # How to use
//!     Just use it! The main idea is simplicity. Enjoy!
//! ## Example
//! ```
//! ```

pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::*;
    use std::collections::HashMap;

    #[test]
    fn foo() {
        let x = Variable::new("x");
        let mut expr = Add::new(Box::new(x.clone()), Box::new(x.clone()));
        // expr.brace();
        expr = expr + x;
        let mut args = HashMap::new();
        args.insert("x", 10.0);
        println!("{}", expr);
        println!("{}", expr.eval_args(&args).unwrap());
    }
}
