# Krabby 🦀 - Rust Tutorial: From Zero to Hero

A comprehensive Rust programming tutorial that takes you from beginner to advanced concepts with hands-on examples and exercises.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Syntax and Concepts](#basic-syntax-and-concepts)
3. [Variables and Data Types](#variables-and-data-types)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Ownership and Borrowing](#ownership-and-borrowing)
7. [Structs and Enums](#structs-and-enums)
8. [Traits](#traits)
9. [Error Handling](#error-handling)
10. [Collections](#collections)
11. [Advanced Topics](#advanced-topics)
12. [Practice Problems](#practice-problems)

## Getting Started

### Installation

1. Install Rust via rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Add Cargo to your PATH:
```bash
source ~/.cargo/env
```

3. Verify installation:
```bash
rustc --version
cargo --version
```

### Running This Tutorial

Clone this repository and run the examples:

```bash
git clone https://github.com/SakuBoyz/krabby.git
cd krabby
cargo test
cargo run --example basic_syntax
```

## Basic Syntax and Concepts

### Hello World

Every Rust program starts with a `main` function:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Comments

```rust
// This is a line comment

/*
 * This is a block comment
 * that spans multiple lines
 */

/// This is a documentation comment
/// for the following item
```

### Printing and Formatting

```rust
fn main() {
    // Basic printing
    println!("Hello, world!");
    
    // Printing with variables
    let name = "Rust";
    println!("Hello, {}!", name);
    
    // Multiple variables
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
    
    // Named parameters
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
}
```

## Variables and Data Types

### Variables and Mutability

```rust
fn main() {
    // Immutable by default
    let x = 5;
    // x = 6; // This would cause a compile error
    
    // Mutable variables
    let mut y = 5;
    y = 6; // This is fine
    
    // Constants (always immutable)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    // Shadowing
    let x = x + 1;
    let x = x * 2; // x is now 12
}
```

### Scalar Types

```rust
fn main() {
    // Integers
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    
    // Floating point
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32
    
    // Boolean
    let t = true;
    let f: bool = false;
    
    // Character
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

### Compound Types

```rust
fn main() {
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    let five_hundred = tup.0; // Access by index
    
    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Type annotation
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    
    // Array access
    let first = a[0];
    let second = a[1];
}
```

## Functions

### Basic Functions

```rust
fn main() {
    println!("Hello, world!");
    
    another_function();
    function_with_parameter(5);
    let sum = add(5, 3);
    println!("The sum is: {}", sum);
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon = return value
}

// Alternative with explicit return
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}
```

### Expressions vs Statements

```rust
fn main() {
    // Statement (doesn't return a value)
    let y = 6;
    
    // Expression (returns a value)
    let x = {
        let y = 3;
        y + 1 // No semicolon = expression
    };
    
    println!("The value of x is: {}", x);
}
```

## Control Flow

### If Expressions

```rust
fn main() {
    let number = 6;
    
    // Basic if
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // Multiple conditions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
}
```

### Loops

```rust
fn main() {
    // Infinite loop
    loop {
        println!("again!");
        break; // Remove this to loop forever
    }
    
    // Loop with return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    
    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    
    // Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
```

## Ownership and Borrowing

### Ownership Rules

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

```rust
fn main() {
    // s is not valid here, it's not yet declared
    {
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid
    
    // String type (heap allocated)
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
    
    // Move semantics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This would cause a compile error
    
    // Clone for deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### References and Borrowing

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // Borrowing
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't own the String, so nothing happens

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Slices

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    let hello = &s[..5]; // Same as [0..5]
    let world = &s[6..]; // Same as [6..len]
    let whole = &s[..]; // Same as [0..len]
    
    let first_word = first_word(&s);
    println!("First word: {}", first_word);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

## Structs and Enums

### Structs

```rust
// Basic struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

impl User {
    // Associated function (constructor)
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    
    // Method
    fn is_active(&self) -> bool {
        self.active
    }
    
    // Mutable method
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    
    // Using constructor
    let user2 = User::new(
        String::from("test@example.com"),
        String::from("testuser"),
    );
    
    // Struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    
    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Enums

```rust
// Basic enum
enum IpAddrKind {
    V4,
    V6,
}

// Enum with data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Complex enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### Option and Pattern Matching

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // Pattern matching with match
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        Some(n) => println!("Got {}", n),
        None => println!("Got nothing"),
    }
    
    // if let (concise control flow)
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

## Traits

Traits define shared behavior in an abstract way. See our [supertrait example](src/examples/supertrait.rs) for advanced trait usage.

```rust
// Basic trait
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify3<T: Summary + Display>(item: &T) {
    // ...
}

// Where clause for complex bounds
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Error Handling

### Recoverable Errors with Result

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Basic Result handling
    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    
    // Using unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// Propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter with chaining
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### Unrecoverable Errors with panic!

```rust
fn main() {
    // Explicit panic
    panic!("crash and burn");
    
    // Panic from invalid array access
    let v = vec![1, 2, 3];
    v[99]; // This will panic
}
```

## Collections

### Vectors

```rust
fn main() {
    // Creating vectors
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    
    // Adding elements
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    // Reading elements
    let v = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2]; // Panics if index doesn't exist
    println!("The third element is {}", third);
    
    let third: Option<&i32> = v.get(2); // Returns None if index doesn't exist
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Iterating
    for i in &v {
        println!("{}", i);
    }
    
    // Mutating while iterating
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
```

### Strings

```rust
fn main() {
    // Creating strings
    let mut s = String::new();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    
    // Updating strings
    let mut s = String::from("foo");
    s.push_str("bar"); // Appends a string slice
    s.push('!'); // Appends a single character
    
    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used
    
    // Format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    // Iterating over strings
    for c in "Зд".chars() {
        println!("{}", c);
    }
    
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
```

### Hash Maps

```rust
use std::collections::HashMap;

fn main() {
    // Creating hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Creating from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Updating values
    let mut scores = HashMap::new();
    
    // Overwriting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
```

## Advanced Topics

### Lifetimes

```rust
// Lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

### Generics

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Specific implementation for f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
}
```

### Closures

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Basic closure
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    // Closure capturing environment
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    
    // Using closures with iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    // Filter example
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    
    let in_my_size = shoes_in_size(shoes, 10);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

### Smart Pointers

```rust
use std::rc::Rc;
use std::cell::RefCell;

// Box<T> for heap allocation
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Rc<T> for multiple ownership
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
```

## Practice Problems

This repository includes practice problems to help you apply what you've learned:

### LeetCode Problems

Check out our [LeetCode solutions](src/leetcode/) including:
- [Reverse Integer](src/leetcode/reverse_int.rs) - Practice with integer manipulation and overflow handling

### Example Projects

Explore our [examples](src/examples/) including:
- [Supertrait Example](src/examples/supertrait.rs) - Advanced trait composition and inheritance

## Running Tests

Test your understanding by running the included tests:

```bash
# Run all tests
cargo test

# Run specific test module
cargo test examples::supertrait

# Run tests with output
cargo test -- --nocapture
```

## Building and Running

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run examples
cargo run --example <example_name>

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Next Steps

After completing this tutorial, consider exploring:

1. **Async Programming** - Learn about `async`/`await` and the Tokio runtime
2. **Web Development** - Frameworks like Actix-web, Warp, or Axum
3. **Systems Programming** - Low-level programming and unsafe Rust
4. **GUI Development** - Libraries like egui, iced, or tauri
5. **Game Development** - Engines like Bevy or Amethyst
6. **WebAssembly** - Compiling Rust to run in browsers
7. **Embedded Programming** - Programming microcontrollers with embedded Rust

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced topics
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

Happy coding! 🦀
