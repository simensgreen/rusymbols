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
//! ```rust
//! use rusymbols::*;
//! use std::collections::HashMap;
//!
//! let z = Variable("z");
//! let expr = X.pow(Y).add(TWO).mul(z).sub(Value(10.0));
//! let vars =
//! {
//!     let mut tmp = HashMap::new();
//!     tmp.insert(X, 10.0);
//!     tmp.insert(Y, 3.0);
//!     tmp.insert(z, 5.0);
//!     tmp
//! };
//!
//! assert_eq!(expr.to_string(), "(x ^ y + 2) * z - 10");
//! assert_eq!(expr.eval_vars(&vars), 5000.0);
//!
//! ```

mod core;

pub use crate::core::*;