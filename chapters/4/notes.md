#Chapter 4

<!--toc:start-->
- [01 - Ownership](#01-ownership)
  - [What is Ownership?](#what-is-ownership)
  - [Ownership as a Discipline for Memory Safety](#ownership-as-a-discipline-for-memory-safety)
  - [Variables Live in the Stack](#variables-live-in-the-stack)
  - [Boxes Live in the Heap](#boxes-live-in-the-heap)
  - [A Box's Owner Manages Deallocation](#a-boxs-owner-manages-deallocation)
  - [Collections Use Boxes](#collections-use-boxes)
<!--toc:end-->

## 01 - Ownership

### What is Ownership?

- Executed in order
- Variables live on the stack
- Copying/shadowing variables results in n+1 memory usage
  - To solve, we can reference the first variable using a pointer

### Ownership as a Discipline for Memory Safety

- Memory is not thought of as RAM or an array of bytes
  - RAM is too general/high-level as we will need to understand the impact of pointers
  - An array of bytes is too specific/concrete as Rust doesn't allow you to interpret memory as an array of bytes

### Variables Live in the Stack

- Variables live in `frames`
  - A mapping from variables to values within a single scope, such as a function.
  - Frames are organized into a `stack`, called such because the most recent frame added is always the next frame that would need freed
- Function calls from within functions will allocate a frame for itself, Ex:

```rust
fn main() {
    let n = 5; // Frame L1
    let y = plus_one(n); // Frame L3
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Frame L2
}
```

### Boxes Live in the Heap

Copying data can be expensive:

```rust
// Create an array of 1 million elements in frame L1
let a = [0; 1_000_000];
// Use `a` to create a new array resulting in 2 million elements in frame L2
let b = a;
```

We can use `std::boxed::Box` [wiki](https://doc.rust-lang.org/std/boxed/index.html) to easily create space on the heap and use a pointer to the location instead of the full data.

```rust
// Create an array of 1 million elements in the heap,
// and set `a` as a pointer to the data.
let a = Box::new([0; 1_000_000]);
// Set `b` to also be a pointer to the same data on the heap
let b = a;
```

### A Box's Owner Manages Deallocation

- When a variable is assigned to something owned, it becomes the owner
- When a function ends, the arguments are deallocated

### Collections Use Boxes

- Boxes are used by Rust dats structures like Vec, String, and HashMap:
```rust
let stringy = String::from("foo"); // Uses a Box to hold the string
```
