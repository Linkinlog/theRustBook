# Chapter 6

<!--toc:start-->

- [Chapter 6](#chapter-6)
  - [01](#01) - [Enums and Pattern Matching](#enums-and-pattern-matching) - [Defining an Enum](#defining-an-enum) - [Enum Values](#enum-values) - [The Option Enum and Its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
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
