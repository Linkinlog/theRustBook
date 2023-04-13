# Chapter 3 - Common Programming Concepts

### Synopsis
This chapter mainly goes over programming concepts and how they work in rust. Variables, basic types, control flow, etc. [Reserved keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html).

### Variables and Mutability
- Variables are immutable by default of course
- Constants are actually constant
    - Can be declared in any scope incl. global
    - must be set to a constant, not a result
    - Should be all uppercase and underscore-case

### Quiz
When creating new variables with `let`, they will be scoped like so:
```rust
let mut x: u32 = 1;
{
    let mut x = x;
    x += 2;
}
println!("{x}"); // 1
```
Where the mutation of x happens inside a scope that does not affect the outer scope

### Data Types
- Scalar and compound
- Inferred well until it's not, and then we need to annotate the types
#### Scalar
- Single value
- Integers, floats, boolean, chars
#### Integer overflow
- If we specify a `u8` value for example, it can only hold values between 0-255
- In debug mode:
  - We panic
- In `--release` mode:
  - We use "Twos compliment wrapping"
    - Essentially wrap the number around
    - Since `u8` holds a max of 255, if we tried to store a value of 256 then the value would end up being 1 as it "wrapped around"

### Functions
- snake case
- `fn function_name() {`
- > In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.
- > In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line. For comments that extend beyond a single line, you’ll need to include // on each line, like this:
```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```
### Control Flow
- No parens are needed in conditional statements
- Conditionals **need** booleans, cannot just be truthy/falsy
- You can return values when `break`ing loops

### Gotcha areas
- `let a = [5;10]; // a = 50;`
  - this syntax means 5 repeated 10 times
