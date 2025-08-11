//! Structs and Enums examples
//! 
//! This module demonstrates:
//! - Defining and using structs
//! - Methods and associated functions
//! - Tuple structs and unit structs
//! - Enums and pattern matching
//! - Option enum and error handling

// Basic struct definition
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl User {
    /// Associated function (constructor) - called with User::new()
    pub fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    
    /// Method - called with instance.method()
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Mutable method - can modify self
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    
    /// Method that takes ownership of self
    pub fn into_username(self) -> String {
        self.username
    }
    
    /// Method with parameters
    pub fn change_email(&mut self, new_email: String) {
        self.email = new_email;
    }
    
    /// Method that returns a reference to avoid cloning
    pub fn get_email(&self) -> &str {
        &self.email
    }
}

// Tuple structs
#[derive(Debug, PartialEq)]
pub struct Color(pub i32, pub i32, pub i32);

#[derive(Debug, PartialEq)]
pub struct Point(pub i32, pub i32, pub i32);

// Unit struct (no fields)
#[derive(Debug)]
pub struct AlwaysEqual;

/// Rectangle struct for area calculation example
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

/// Demonstrates basic struct usage
pub fn struct_example() {
    println!("=== Structs ===");
    
    // Creating struct instances
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Accessing and modifying fields
    user1.email = String::from("anotheremail@example.com");
    println!("User1: {:?}", user1);
    
    // Using constructor
    let user2 = User::new(
        String::from("test@example.com"),
        String::from("testuser"),
    );
    println!("User2: {:?}", user2);
    
    // Struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2 // Use remaining fields from user2
    };
    println!("User3: {:?}", user3);
    
    // Using methods
    println!("User1 is active: {}", user1.is_active());
    user1.deactivate();
    println!("User1 is active after deactivation: {}", user1.is_active());
    
    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: {:?}", black);
    println!("Origin point: {:?}", origin);
    
    // Unit struct
    let subject = AlwaysEqual;
    println!("Always equal: {:?}", subject);
}

/// Demonstrates rectangle area calculation
pub fn rectangle_example() {
    println!("\n=== Rectangle Example ===");
    
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    
    println!("rect1 is {:?}", rect1);
    println!("The area of rect1 is {} square pixels.", rect1.area());
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(25);
    println!("Square: {:?}, area: {}", square, square.area());
}

// Enums
#[derive(Debug, PartialEq)]
pub enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug, PartialEq)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        match self {
            Message::Quit => println!("The Quit variant"),
            Message::Move { x, y } => println!("Move to coordinates x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red: {}, green: {}, blue: {}", r, g, b)
            }
        }
    }
}

/// Demonstrates enum usage
pub fn enum_example() {
    println!("\n=== Enums ===");
    
    // Basic enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IPv4: {:?}, IPv6: {:?}", four, six);
    
    // Enums with data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
    
    // Complex enums
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("hello"));
    let color_msg = Message::ChangeColor(255, 0, 0);
    
    quit.call();
    move_msg.call();
    write_msg.call();
    color_msg.call();
}

/// Demonstrates Option enum and pattern matching
pub fn option_example() {
    println!("\n=== Option and Pattern Matching ===");
    
    // Option enum examples
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    // Pattern matching with match
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        Some(n) => println!("Got {}", n),
        None => println!("Got nothing"),
    }
    
    // Using match with enums
    let ip = IpAddr::V4(192, 168, 1, 1);
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 address: {}", addr);
        }
    }
    
    // if let syntax for concise matching
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    
    // Using Option in functions
    let numbers = vec![1, 2, 3, 4, 5];
    println!("First element: {:?}", get_first(&numbers));
    println!("Element at index 10: {:?}", get_at_index(&numbers, 10));
    
    // Working with Option values
    let some_value = Some(42);
    let doubled = double_option(some_value);
    println!("Doubled value: {:?}", doubled);
    
    let no_value: Option<i32> = None;
    let doubled_none = double_option(no_value);
    println!("Doubled None: {:?}", doubled_none);
}

/// Get the first element of a vector
pub fn get_first<T>(vec: &Vec<T>) -> Option<&T> {
    if vec.is_empty() {
        None
    } else {
        Some(&vec[0])
    }
}

/// Get element at specific index
pub fn get_at_index<T>(vec: &Vec<T>, index: usize) -> Option<&T> {
    if index < vec.len() {
        Some(&vec[index])
    } else {
        None
    }
}

/// Double an optional number
pub fn double_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

/// Alternative implementation using map
pub fn double_option_with_map(x: Option<i32>) -> Option<i32> {
    x.map(|i| i * 2)
}

/// Demonstrates Result enum for error handling
pub fn result_example() {
    println!("\n=== Result and Error Handling ===");
    
    // Using Result for division
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    match result1 {
        Ok(value) => println!("10.0 / 2.0 = {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    match result2 {
        Ok(value) => println!("10.0 / 0.0 = {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using unwrap_or for default values
    let safe_result = divide(15.0, 3.0).unwrap_or(0.0);
    println!("Safe division result: {}", safe_result);
    
    let safe_result = divide(15.0, 0.0).unwrap_or(0.0);
    println!("Safe division with error: {}", safe_result);
}

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
        }
    }
}

pub fn divide(x: f64, y: f64) -> Result<f64, MathError> {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

/// Run all struct and enum examples
pub fn run_all_examples() {
    println!("🦀 Structs and Enums Examples 🦀\n");
    
    struct_example();
    rectangle_example();
    enum_example();
    option_example();
    result_example();
    
    println!("\n✅ All struct and enum examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            String::from("test@example.com"),
            String::from("testuser"),
        );
        
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
        assert!(user.active);
        assert_eq!(user.sign_in_count, 1);
    }

    #[test]
    fn test_user_methods() {
        let mut user = User::new(
            String::from("test@example.com"),
            String::from("testuser"),
        );
        
        assert!(user.is_active());
        user.deactivate();
        assert!(!user.is_active());
        
        user.change_email(String::from("new@example.com"));
        assert_eq!(user.get_email(), "new@example.com");
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(30, 50);
        assert_eq!(rect.area(), 1500);
        
        let small_rect = Rectangle::new(10, 40);
        let large_rect = Rectangle::new(60, 45);
        
        assert!(rect.can_hold(&small_rect));
        assert!(!rect.can_hold(&large_rect));
        
        let square = Rectangle::square(25);
        assert_eq!(square.width, 25);
        assert_eq!(square.height, 25);
        assert_eq!(square.area(), 625);
    }

    #[test]
    fn test_enums() {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        
        assert_eq!(home, IpAddr::V4(127, 0, 0, 1));
        assert_eq!(loopback, IpAddr::V6(String::from("::1")));
    }

    #[test]
    fn test_option_functions() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        assert_eq!(get_first(&numbers), Some(&1));
        assert_eq!(get_at_index(&numbers, 2), Some(&3));
        assert_eq!(get_at_index(&numbers, 10), None);
        
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(get_first(&empty_vec), None);
    }

    #[test]
    fn test_double_option() {
        assert_eq!(double_option(Some(5)), Some(10));
        assert_eq!(double_option(None), None);
        
        assert_eq!(double_option_with_map(Some(5)), Some(10));
        assert_eq!(double_option_with_map(None), None);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
        
        match divide(10.0, 0.0) {
            Err(MathError::DivisionByZero) => (), // Expected
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_tuple_structs() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        
        assert_eq!(black.0, 0);
        assert_eq!(black.1, 0);
        assert_eq!(black.2, 0);
        
        assert_eq!(origin.0, 0);
        assert_eq!(origin.1, 0);
        assert_eq!(origin.2, 0);
    }
}