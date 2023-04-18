# Chapter 4

<!--toc:start-->
- [Chapter 4](#chapter-4)
  - [01 - Ownership](#01-ownership)
    - [What is Ownership?](#what-is-ownership)
    - [Ownership as a Discipline for Memory Safety](#ownership-as-a-discipline-for-memory-safety)
    - [Variables Live in the Stack](#variables-live-in-the-stack)
    - [Boxes Live in the Heap](#boxes-live-in-the-heap)
    - [A Box's Owner Manages Deallocation](#a-boxs-owner-manages-deallocation)
    - [Collections Use Boxes](#collections-use-boxes)
  - [02 - References and Borrowing](#02-references-and-borrowing)
    - [References are Non-Owning Pointers](#references-are-non-owning-pointers)
    - [Dereferencing a Pointer Accesses Its Data](#dereferencing-a-pointer-accesses-its-data)
    - [Rust Avoids Simultaneous Aliasing and Mutation](#rust-avoids-simultaneous-aliasing-and-mutation)
    - [References Change Permissions on Paths](#references-change-permissions-on-paths)
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

## 02 - References and Borrowing

Using the heap is considered a "move only" API, which can be inconvenient:

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2); // Errors here as m1/m2 got moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}", g1, g2);
}
```

To fix this we can return the ownership at the end of the greet function, however that often leads to a much more verbose syntax.

### References are Non-Owning Pointers

Rust lets us use pointers, so we can borrow m1 and m2:

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2); // Passing by reference / borrowing
    let s = format!("{} {}", m1, m2)
}
fn greet(g1: &String, g2: &String) {
    // g1 / g2 are references to m1/m2
    println!("{} {}", g1, g2);
}
```

- References are **non-owning pointers**, because they do not own the data they point to. Seems like this allows for automatic cleanup as the only thing that needs to be removed to clear memory is m1 or m2, and we can't try to access what isn't there anymore.

### Dereferencing a Pointer Accesses Its Data

Dereferencing a pointer allows us to use the value that resides in the heap instead of just a pointer to the value. Consider the following:

```rust
fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                             // so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;        // so only one dereference is needed to read it
}
```

This is not used very much though, as Rust references and dereferences implicitly in certain cases, such as with the dot operator shown here: `x.abs();`

### Rust Avoids Simultaneous Aliasing and Mutation

- Doing a `push` to a vector that is at capacity will result in a new vector being created, the new vector to be filled with the old vectors data, and then the old vector deallocated

### References Change Permissions on Paths

- A path is basically anything to the left of an assignment (`=`)
- There are 3 kinds of permissions:
  - Read - data can be copied to another location
  - Write - Data can be mutated in-place
  - Own - Data can be moved or dropped
- Default is R/O
- `mut` adds W
- References can temporarily remove these permissions

## 03 - Fixing Ownership Errors

 A common ownership issue can occcur whenever we make some new data within a function and return a reference to the data. Ex:
 ```rust
 fn return_a_string() -> &String {
     let a = String::from("Hello World");
     &a
 }
 ```

To solve this we have some options:
1. Return the string itself, moving ownership. 
```rust
fn ex_one() -> String {
    let a = String::from("Hello World");
    a
}
```
2. Return a string literal if we never intend to change the string's value. 
3. Defer lifetime-checking to the runtime by using garbage collection, for example we can use a [reference-counted pointer](https://doc.rust-lang.org/std/rc/index.html). 
4. Provide a "slot" where we take in a mutable reference to a string and then modify it within our function, this makes it, so the memory needs to be taken care of before/during the function call instead of within the function. 
