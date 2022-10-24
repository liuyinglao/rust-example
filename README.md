# Differences


1. `mut` keyword is needed for a variable to be mutable.
2. Match expression returns a value.
3. `println!` macro is used for output.
4. `vec!` is used to create vector.
5. Debug placeholder `{:?}` to be replaced by whichever type.
6. Variable is owned by functions, and cleaned up when its owning function ends. Ownership is passed when it is called by another funciton.
7. To not pass ownership, `&` is used to denote borrowing.
8. `Result` type can hold a value or a error. A value is `unwrap()` from the returned `Result`.

