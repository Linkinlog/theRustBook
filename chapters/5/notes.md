# Chapter 5 - Using Structs to Structure Related Data

<!--toc:start-->

- [Chapter 5 - Using Structs to Structure Related Data](#chapter-5-using-structs-to-structure-related-data)
  - [01](#01)
    - [Defining and Instantiating Structs](#defining-and-instantiating-structs)
    - [Using Tuple Structs without Named Fields to Create Different Types](#using-tuple-structs-without-named-fields-to-create-different-types)
    - [Unit-Like Structs Without Any Fields](#unit-like-structs-without-any-fields)
  - [02](#02)
    - [Example function for calculating area](#example-function-for-calculating-area)
  - [03](#03) - [Method Syntax](#method-syntax)
  <!--toc:end-->

## 01

### Defining and Instantiating Structs

- Structs can be thought of similarly to objects in other languages or structs in Go.
- Similar to tuples but with named values
  Ex:

```rust
struct Car {
    fast: bool,
    make: string,
    model: string,
    year: u32,
};
```

- Fields can be accessed/modified with dot syntax: `Car.fast = false;`
- Entire struct needs to be mutable if we need to edit a field.
- When instantiating a struct we can use shorthand syntax if the variables are named the same.

### Using Tuple Structs without Named Fields to Create Different Types

Can be useful when needing to make different types, for example color RGBs:

```rust
struct Color(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
}
```

### Unit-Like Structs Without Any Fields

- Structs can be defined without any fields, these are called unit-like.

## 02

### Example function for calculating area

- Made a function to calculate the area, in ./area/src/main.rs (original below)
- Uses a `Rectangle` struct
- Fairly straightforward, only gotcha was I need to not pass variables but pass references

```rust
struct Rectangle {
    length: usize,
    width: usize,
}
fn get_area(rec: Rectangle) -> usize {
    rec.length * rec.width
}
fn main() {
    let rec = Rectangle {
        length: 50,
        width: 30,
    };
    let area = get_area(rec);
    println!("{}", area);
}
```

## 03

### Method Syntax
