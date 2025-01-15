# Multiple Mutable Borrows in Rust
This repository demonstrates a common error in Rust: creating multiple mutable borrows of the same variable.  Rust's borrow checker prevents this to avoid data races and ensure memory safety. 

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file shows how to fix it using techniques like cloning or using interior mutability.