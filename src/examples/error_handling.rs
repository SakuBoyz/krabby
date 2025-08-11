//! Error Handling examples
//! 
//! This module demonstrates Rust's approach to error handling:
//! - Unrecoverable errors with panic!
//! - Recoverable errors with Result<T, E>
//! - The ? operator for error propagation
//! - Custom error types
//! - Error conversion and chaining

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::fmt;
use std::error::Error;

/// Demonstrates panic! for unrecoverable errors
pub fn panic_example() {
    println!("=== Panic for Unrecoverable Errors ===");
    
    // Explicit panic
    println!("About to demonstrate panic (this will be caught):");
    
    // We'll demonstrate this without actually panicking in tests
    println!("This would panic: panic!(\"Something went wrong!\")");
    
    // Array bounds checking that would panic
    let v = vec![1, 2, 3];
    println!("Vector: {:?}", v);
    println!("Accessing v[1]: {}", v[1]); // Safe access
    println!("Accessing v[10] would panic! (out of bounds)");
    
    // We can set environment variable to see backtrace:
    // RUST_BACKTRACE=1 cargo run
    println!("Set RUST_BACKTRACE=1 to see backtraces on panic");
}

/// Demonstrates basic Result usage
pub fn basic_result_example() {
    println!("\n=== Basic Result Usage ===");
    
    // Attempting to open a file
    println!("Attempting to open 'hello.txt':");
    let greeting_file_result = File::open("hello.txt");
    
    match greeting_file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Problem opening the file: {:?}", error),
    }
    
    // More specific error handling
    println!("\nMore specific error handling:");
    let greeting_file_result = File::open("hello.txt");
    
    let _greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("File found and opened!");
            file
        },
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found, this is expected in our example");
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("File created successfully!");
                        fc
                    },
                    Err(e) => {
                        println!("Problem creating the file: {:?}", e);
                        return;
                    }
                }
            },
            other_error => {
                println!("Problem opening the file: {:?}", other_error);
                return;
            }
        },
    };
    
    println!("File operations completed successfully!");
}

/// Demonstrates unwrap and expect
pub fn unwrap_expect_example() {
    println!("\n=== Unwrap and Expect ===");
    
    // Using unwrap_or_else for more control
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            println!("File not found, creating it...");
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    println!("File handle obtained: {:?}", greeting_file);
    
    // Examples of unwrap and expect (we won't actually call them on potentially failing operations)
    println!("unwrap() will panic if Result is Err");
    println!("expect() will panic with a custom message if Result is Err");
    
    // Safe examples with Some/None
    let some_value = Some(42);
    let value = some_value.unwrap();
    println!("Unwrapped value: {}", value);
    
    let some_other_value = Some(100);
    let other_value = some_other_value.expect("Value should exist");
    println!("Expected value: {}", other_value);
    
    // unwrap_or provides default values
    let none_value: Option<i32> = None;
    let default_value = none_value.unwrap_or(0);
    println!("Default value when None: {}", default_value);
}

/// Function that propagates errors using ?
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/// Shorter version using method chaining
pub fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// Even shorter using fs::read_to_string
pub fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

/// Demonstrates the ? operator
pub fn question_mark_operator_example() {
    println!("\n=== The ? Operator ===");
    
    // Create a test file first
    std::fs::write("username.txt", "alice123").unwrap_or_else(|_| {
        println!("Could not create test file, but continuing with example");
    });
    
    // Try reading with different methods
    match read_username_from_file() {
        Ok(username) => println!("Username (long version): {}", username.trim()),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    match read_username_from_file_short() {
        Ok(username) => println!("Username (short version): {}", username.trim()),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    match read_username_from_file_shortest() {
        Ok(username) => println!("Username (shortest version): {}", username.trim()),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    // The ? operator can only be used in functions that return Result or Option
    println!("The ? operator automatically propagates errors up the call stack");
}

/// Custom error type
#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
    NegativeSquareRoot,
    ParseError(ParseIntError),
}

impl PartialEq for CalculatorError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CalculatorError::DivisionByZero, CalculatorError::DivisionByZero) => true,
            (CalculatorError::NegativeSquareRoot, CalculatorError::NegativeSquareRoot) => true,
            (CalculatorError::ParseError(_), CalculatorError::ParseError(_)) => true,
            _ => false,
        }
    }
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalculatorError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
            CalculatorError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for CalculatorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CalculatorError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

// Convert ParseIntError to CalculatorError
impl From<ParseIntError> for CalculatorError {
    fn from(error: ParseIntError) -> Self {
        CalculatorError::ParseError(error)
    }
}

/// Calculator functions that return custom errors
pub fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
    if b == 0.0 {
        Err(CalculatorError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn square_root(x: f64) -> Result<f64, CalculatorError> {
    if x < 0.0 {
        Err(CalculatorError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

pub fn parse_and_double(s: &str) -> Result<i32, CalculatorError> {
    let num: i32 = s.parse()?; // ? automatically converts ParseIntError to CalculatorError
    Ok(num * 2)
}

/// Demonstrates custom error types
pub fn custom_error_example() {
    println!("\n=== Custom Error Types ===");
    
    // Test division
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test square root
    match square_root(16.0) {
        Ok(result) => println!("√16 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match square_root(-4.0) {
        Ok(result) => println!("√(-4) = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test parsing with error conversion
    match parse_and_double("42") {
        Ok(result) => println!("Parse and double '42': {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_and_double("not_a_number") {
        Ok(result) => println!("Parse and double 'not_a_number': {}", result),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
    }
}

/// Demonstrates error handling patterns
pub fn error_handling_patterns() {
    println!("\n=== Error Handling Patterns ===");
    
    // Pattern 1: Early return with ?
    fn process_numbers(a: &str, b: &str) -> Result<i32, CalculatorError> {
        let num_a = a.parse::<i32>()?;
        let num_b = b.parse::<i32>()?;
        Ok(num_a + num_b)
    }
    
    match process_numbers("5", "10") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error processing numbers: {}", e),
    }
    
    // Pattern 2: Combining Results
    let results = vec![
        parse_and_double("1"),
        parse_and_double("2"),
        parse_and_double("3"),
    ];
    
    // Collect all successful results
    let successful: Vec<i32> = results.into_iter()
        .filter_map(|r| r.ok())
        .collect();
    println!("Successful results: {:?}", successful);
    
    // Pattern 3: Converting errors to options
    let maybe_number = "42".parse::<i32>().ok();
    let maybe_invalid = "abc".parse::<i32>().ok();
    
    println!("Maybe number: {:?}", maybe_number);
    println!("Maybe invalid: {:?}", maybe_invalid);
    
    // Pattern 4: Using map_err to transform errors
    let result = "not_a_number".parse::<i32>()
        .map_err(|_| "Could not parse as number");
    
    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Custom error: {}", e),
    }
    
    // Pattern 5: Combining operations with and_then
    let result = "5".parse::<i32>()
        .and_then(|n| {
            if n > 0 {
                Ok(n * 2)
            } else {
                Err("Number must be positive".parse::<i32>().unwrap_err())
            }
        });
    
    match result {
        Ok(doubled) => println!("Doubled positive number: {}", doubled),
        Err(e) => println!("Error in chain: {:?}", e),
    }
}

/// Demonstrates working with multiple error types
pub fn multiple_error_types_example() {
    println!("\n=== Multiple Error Types ===");
    
    // Function that can return different error types
    fn read_and_parse_number(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(filename)?;
        let number = contents.trim().parse()?;
        Ok(number)
    }
    
    // Create a test file
    std::fs::write("number.txt", "42").unwrap_or(());
    
    match read_and_parse_number("number.txt") {
        Ok(num) => println!("Read number from file: {}", num),
        Err(e) => println!("Error reading/parsing number: {}", e),
    }
    
    match read_and_parse_number("nonexistent.txt") {
        Ok(num) => println!("Read number: {}", num),
        Err(e) => println!("Error (file not found): {}", e),
    }
    
    // Using Result<T, Box<dyn Error>> for functions that can have multiple error types
    println!("Box<dyn Error> allows functions to return any error type");
}

/// Run all error handling examples
pub fn run_all_examples() {
    println!("🦀 Error Handling Examples 🦀\n");
    
    panic_example();
    basic_result_example();
    unwrap_expect_example();
    question_mark_operator_example();
    custom_error_example();
    error_handling_patterns();
    multiple_error_types_example();
    
    // Clean up test files
    let _ = std::fs::remove_file("hello.txt");
    let _ = std::fs::remove_file("username.txt");
    let _ = std::fs::remove_file("number.txt");
    
    println!("\n✅ All error handling examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_function() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
        
        match divide(10.0, 0.0) {
            Err(CalculatorError::DivisionByZero) => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_square_root_function() {
        assert_eq!(square_root(16.0), Ok(4.0));
        assert_eq!(square_root(0.0), Ok(0.0));
        assert!(square_root(-1.0).is_err());
        
        match square_root(-1.0) {
            Err(CalculatorError::NegativeSquareRoot) => (),
            _ => panic!("Expected NegativeSquareRoot error"),
        }
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert_eq!(parse_and_double("0"), Ok(0));
        assert_eq!(parse_and_double("-3"), Ok(-6));
        
        assert!(parse_and_double("not_a_number").is_err());
        assert!(parse_and_double("").is_err());
    }

    #[test]
    fn test_error_conversion() {
        // Test that ParseIntError is converted to CalculatorError
        let result = parse_and_double("invalid");
        match result {
            Err(CalculatorError::ParseError(_)) => (),
            _ => panic!("Expected ParseError variant"),
        }
    }

    #[test]
    fn test_option_unwrap_or() {
        let some_value = Some(42);
        assert_eq!(some_value.unwrap_or(0), 42);
        
        let none_value: Option<i32> = None;
        assert_eq!(none_value.unwrap_or(0), 0);
    }

    #[test]
    fn test_result_map_err() {
        let result: Result<i32, &str> = Err("original error");
        let mapped = result.map_err(|_| "mapped error");
        
        match mapped {
            Err("mapped error") => (),
            _ => panic!("Error was not mapped correctly"),
        }
    }

    #[test]
    fn test_file_operations() {
        // Test file creation and reading
        let test_content = "test content";
        let test_file = "test_file.txt";
        
        // Create file
        std::fs::write(test_file, test_content).expect("Should write file");
        
        // Read file
        let content = std::fs::read_to_string(test_file).expect("Should read file");
        assert_eq!(content, test_content);
        
        // Clean up
        std::fs::remove_file(test_file).expect("Should remove file");
    }
}