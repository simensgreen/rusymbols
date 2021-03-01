# rusymbols
# Note:
The resulting crate architecture does not correspond to the main ideas, therefore it will be reviewed and reassembled. 
Please wait for a more stable version
 ## About

 rusymbols is a Rust crate for symbolic mathematics. It aims to become a full-featured computer
 algebra system (CAS) while keeping the code as simple as possible in order to be comprehensible
 and easily extensible. rusymbols is written entirely in Rust.

 (maybe some engineers want to learn Rust :))

 The crate is designed to support the Rust language and add the ability to work with complex
 formulas at the speed of Rust, possibly slower than programmed without a high level of
 abstraction. Do not think about their solution, especially since there are formulas that change
 depending on the circumstances.The goal of the project is at least to become similar to SymPy.

 ## Key ideas:
     - Simple.
     - Fast (maybe not at maximum speed, but still fast)
     - Safe
     - Universal (compatibility with nalgebra and standard types (possibly) and other similar crates)

 ## How to use
     Just use it! The main idea is simplicity. Enjoy!
 ### Example
 ```
 use rusymbols::Expression;
 use std::collections::HashMap;

 let x = Expression::new_var("x");
 let two = Expression::new_val(2.0);
 let polynomial = x.clone().pow(two.clone()) - x + two;

 let mut args: HashMap<&str, f64> = HashMap::new();
 args.insert("x", 1.0);

 assert_eq!(polynomial.to_string(), "x**2 - x + 2");
 assert_eq!(polynomial.eval_args(&args).unwrap(), 2.0);
 ```
