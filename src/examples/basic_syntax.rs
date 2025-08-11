//! Basic Rust syntax examples for beginners
//! 
//! This module demonstrates fundamental Rust concepts including:
//! - Variables and mutability
//! - Data types
//! - Functions
//! - Control flow

/// Example demonstrating variables and mutability
pub fn variables_example() {
    println!("=== Variables and Mutability ===");
    
    // Immutable variable (default)
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Mutable variable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is now: {}", y);
    
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    
    // Shadowing
    let x = x + 1; // x is now 6
    let x = x * 2; // x is now 12
    println!("The value of x after shadowing is: {}", x);
}

/// Example demonstrating different data types
pub fn data_types_example() {
    println!("\n=== Data Types ===");
    
    // Integers
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    println!("Decimal: {}, Hex: {}, Octal: {}, Binary: {}", decimal, hex, octal, binary);
    
    // Floating point
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32
    println!("Float x: {}, Float y: {}", x, y);
    
    // Boolean
    let t = true;
    let f: bool = false;
    println!("Boolean t: {}, Boolean f: {}", t, f);
    
    // Character
    let c = 'z';
    let heart_eyed_cat = '😻';
    println!("Character c: {}, Emoji: {}", c, heart_eyed_cat);
    
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("Tuple values: x={}, y={}, z={}", x, y, z);
    println!("Tuple first element: {}", tup.0);
    
    // Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("Array first: {}, second: {}", first, second);
}

/// Example demonstrating functions
pub fn add(x: i32, y: i32) -> i32 {
    x + y // Expression without semicolon
}

pub fn subtract(x: i32, y: i32) -> i32 {
    return x - y; // Explicit return
}

pub fn functions_example() {
    println!("\n=== Functions ===");
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let result = subtract(10, 4);
    println!("10 - 4 = {}", result);
    
    // Function that returns a value from a block
    let x = {
        let y = 3;
        y + 1 // No semicolon = expression that returns value
    };
    println!("Value from block expression: {}", x);
}

/// Example demonstrating control flow
pub fn control_flow_example() {
    println!("\n=== Control Flow ===");
    
    // If expressions
    let number = 6;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }
    
    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    // Loops
    println!("Countdown:");
    let mut counter = 3;
    while counter > 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    println!("Liftoff! 🚀");
    
    // For loop with range
    println!("For loop with range:");
    for i in 1..4 {
        println!("Number: {}", i);
    }
    
    // For loop with array
    let a = [10, 20, 30, 40, 50];
    println!("Array elements:");
    for element in a {
        println!("Value: {}", element);
    }
}

/// Run all basic syntax examples
pub fn run_all_examples() {
    println!("🦀 Welcome to Rust Basic Syntax Examples! 🦀\n");
    
    variables_example();
    data_types_example();
    functions_example();
    control_flow_example();
    
    println!("\n✅ All basic syntax examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_function() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract_function() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(10, 15), -5);
        assert_eq!(subtract(0, 0), 0);
    }

    #[test]
    fn test_examples_run_without_panic() {
        // These functions print to stdout, we just want to ensure they don't panic
        variables_example();
        data_types_example();
        functions_example();
        control_flow_example();
    }
}