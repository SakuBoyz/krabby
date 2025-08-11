//! Ownership and Borrowing examples
//! 
//! This module demonstrates Rust's ownership system including:
//! - Ownership rules and move semantics
//! - References and borrowing
//! - Mutable and immutable references
//! - String slices and array slices

/// Demonstrates basic ownership and move semantics
pub fn ownership_example() {
    println!("=== Ownership and Move Semantics ===");
    
    // Stack-allocated data (Copy types)
    let x = 5;
    let y = x; // x is copied, both x and y are valid
    println!("x = {}, y = {}", x, y);
    
    // Heap-allocated data (Move types)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("s1 = {}", s1); // This would cause a compile error!
    println!("s2 = {}", s2);
    
    // Clone for deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // Both are valid
    
    // Function calls and ownership
    let s = String::from("hello");
    takes_ownership(s); // s is moved into the function
    // println!("{}", s); // This would cause a compile error!
    
    let x = 5;
    makes_copy(x); // x is copied into the function
    println!("x is still valid: {}", x); // x is still valid
    
    // Getting ownership back from functions
    let s1 = gives_ownership(); // Function returns ownership
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved in, ownership returned
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("Function received: {}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("Function received copy: {}", some_integer);
} // some_integer goes out of scope, but nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // Return value is moved to calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Return the string to the calling function
}

/// Demonstrates references and borrowing
pub fn borrowing_example() {
    println!("\n=== References and Borrowing ===");
    
    let s1 = String::from("hello");
    
    // Immutable reference (borrowing)
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid
    
    // Multiple immutable references are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {}", s);
    
    // Mutable reference rules demonstration
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(", world");
        // Only one mutable reference allowed in scope
        // let r2 = &mut s; // This would cause a compile error!
    } // r1 goes out of scope
    
    let r2 = &mut s; // Now we can create another mutable reference
    r2.push('!');
    println!("Final string: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own the String, so nothing is dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/// Demonstrates string slices
pub fn slice_example() {
    println!("\n=== Slices ===");
    
    let s = String::from("hello world");
    
    // String slices
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    let hello2 = &s[..5];   // Same as [0..5]
    let world2 = &s[6..];   // Same as [6..len]
    let whole = &s[..];     // Same as [0..len]
    
    println!("hello: {}", hello);
    println!("world: {}", world);
    println!("hello2: {}", hello2);
    println!("world2: {}", world2);
    println!("whole: {}", whole);
    
    // Using first_word function
    let word = first_word(&s);
    println!("First word: {}", word);
    
    // String literals are slices
    let s = "Hello, world!"; // Type is &str
    let word = first_word_improved(s);
    println!("First word from literal: {}", word);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
    println!("Array slice: {:?}", slice);
    
    assert_eq!(slice, &[2, 3]);
}

/// Find the first word in a string (returns a slice)
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// Improved version that works with both String and &str
fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// Demonstrates common reference patterns and lifetime rules
pub fn reference_patterns_example() {
    println!("\n=== Reference Patterns ===");
    
    // Dangling reference prevention
    // let reference_to_nothing = dangle(); // This would cause a compile error!
    let valid_string = no_dangle();
    println!("Valid string: {}", valid_string);
    
    // Multiple references rules
    let mut s = String::from("hello");
    
    let r1 = &s; // No problem
    let r2 = &s; // No problem (multiple immutable refs allowed)
    println!("{} and {}", r1, r2);
    // Variables r1 and r2 will not be used after this point
    
    let r3 = &mut s; // No problem (immutable refs are no longer used)
    r3.push_str(" world");
    println!("{}", r3);
}

// This function would cause a compile error if uncommented
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // We return a reference to s, but s will be dropped!
} // s goes out of scope and is dropped, so reference would be invalid
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Return the String itself, transferring ownership
}

/// Run all ownership and borrowing examples
pub fn run_all_examples() {
    println!("🦀 Ownership and Borrowing Examples 🦀\n");
    
    ownership_example();
    borrowing_example();
    slice_example();
    reference_patterns_example();
    
    println!("\n✅ All ownership examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        // s is still valid here
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
        
        let s = String::from("hello");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_first_word_improved() {
        // Works with String
        let s = String::from("hello world");
        assert_eq!(first_word_improved(&s), "hello");
        
        // Works with string literal
        assert_eq!(first_word_improved("hello world"), "hello");
        assert_eq!(first_word_improved("hello"), "hello");
        assert_eq!(first_word_improved(""), "");
    }

    #[test]
    fn test_change_function() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_slice_equality() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let hello2 = &s[..5];
        assert_eq!(hello, hello2);
        assert_eq!(hello, "hello");
    }
}