# Chapter 6

<!--toc:start-->

- [Chapter 6](#chapter-6)
  - [01](#01)
    - [Enums and Pattern Matching](#enums-and-pattern-matching)
    - [Defining an Enum](#defining-an-enum)
    - [Enum Values](#enum-values)
    - [The Option Enum and Its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
  - [02](#02) - [The match Control Flow Construct](#the-match-control-flow-construct) - [Patterns that Bind to Values](#patterns-that-bind-to-values) - [Catch-all Patterns and the \_ Placeholder](#catch-all-patterns-and-the-placeholder) - [How Matches Interact with Ownership](#how-matches-interact-with-ownership)
  <!--toc:end-->

## 01

### Enums and Pattern Matching

- Enums are used to define possible types, to use the previous example, a `Rectangle` is a `Shape`

### Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6
}
```

### Enum Values

- Using enum values is like an assosciated function syntax `let four = IpAddrKind::V4;`
  We can set values like such:

```rust
enum IpAddr {
    V4(String),
    V6(u8, u8, u8, u8),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
```

### The Option Enum and Its Advantages Over Null Values

Null values can cause issues when you try to use a null value as a not-null value. These errors with null values are an oversight and rust takes the stance away from null values entirely. Their rebuttal is called the `Option` enum, its signature looks like this:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

This allows someone to return either nothing or something, which handles the main use case for `null` without adding in uncertainty.

## 02

### The match Control Flow Construct

- Consists of arms to do the Matching
- Functions like `match x {` and will go through each arm and evaluate if the left side of the arm is equal to `x` and if so, it runs/returns the right side of the arm.

Ex:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns that Bind to Values

- We can set a variable within the enum to actually hold data, once the enum is evaluated in a match statement, the data will be bound to the variable you used in the match statement.

Consider the following:

```rust
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Found a state quarter from {:?}!", state);
            25
        }
    }
}
```

Since `Quarter` knew about `UsState` we were able to use it in our math statement with no issues, and, we were able to print out the result.

### Catch-all Patterns and the \_ Placeholder

If we set the **last** arm in the match to be a new variable and use it, we can consider this a catch-all.
Consider the following:

```rust
let dice_roll = 9;
match dice_roll {
    5 => do_the_roar(),
    3 => learn_to_type(),
    catchAll => another_function(catchAll),
}
```

If we don't need the value to be passed anywhere we can do the same syntax but replace `catchAll` with `_`.

### How Matches Interact with Ownership

When preforming a match, you will loose read/own to the internal items, for things like options/enums. The idiomatic way around this is to use a reference like so:

```rust
fn main() {
let opt: Option<String> =
    Some(String::from("Hello world"));

// opt became &opt
match &opt {
    Some(s) => println!("Some: {}", s),
    None => println!("None!")
};

println!("{:?}", opt);
}
```
