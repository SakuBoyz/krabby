//! Traits examples demonstrating Rust's trait system
//! 
//! This module demonstrates:
//! - Defining and implementing traits
//! - Default implementations
//! - Trait bounds and where clauses
//! - Trait objects and dynamic dispatch
//! - Operator overloading
//! - Advanced trait patterns

use std::fmt;

/// Basic trait example
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("(Read more from this author...)")
    }
    
    // Default implementation that calls other method
    fn summarize_with_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait as parameter (impl Trait syntax)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
pub fn notify_and_display<T: Summary + fmt::Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item);
}

// Where clause for complex bounds
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: fmt::Display + Clone,
    U: Clone + fmt::Debug,
{
    println!("t: {}", t);
    println!("u: {:?}", u);
    42
}

// Returning types that implement traits
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/// Demonstrates basic trait usage
pub fn basic_traits_example() {
    println!("=== Basic Traits ===");
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());
    
    // Using default implementations
    println!("Tweet author: {}", tweet.summarize_author());
    println!("Article author: {}", article.summarize_author());
    
    println!("Tweet with author: {}", tweet.summarize_with_author());
    println!("Article with author: {}", article.summarize_with_author());
    
    // Using trait parameters
    notify(&tweet);
    notify(&article);
    
    // Function that returns trait
    let returned_tweet = returns_summarizable();
    println!("Returned tweet: {}", returned_tweet.summarize());
}

/// Trait for mathematical operations
pub trait Add<T> {
    type Output;
    fn add(self, other: T) -> Self::Output;
}

/// Point struct for demonstrating operator overloading
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add<Point> for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;
    
    fn add(self, other: i32) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// Implementing standard library traits
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Demonstrates operator overloading
pub fn operator_overloading_example() {
    println!("\n=== Operator Overloading ===");
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    
    // Using our custom Add trait
    let p3 = p1.clone().add(p2.clone());
    println!("Custom add: {} + {} = {}", p1, p2, p3);
    
    let p4 = p1.clone().add(5);
    println!("Add scalar: {} + 5 = {}", p1, p4);
    
    // Using standard library Add trait (+ operator)
    let p5 = p1.clone() + p2.clone();
    println!("Operator +: {} + {} = {}", p1, p2, p5);
}

/// Trait objects and dynamic dispatch
pub trait Draw {
    fn draw(&self);
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            components: Vec::new(),
        }
    }
    
    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }
    
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// Demonstrates trait objects
pub fn trait_objects_example() {
    println!("\n=== Trait Objects ===");
    
    let mut screen = Screen::new();
    
    screen.add_component(Box::new(Circle { radius: 5.0 }));
    screen.add_component(Box::new(Rectangle { width: 10.0, height: 20.0 }));
    screen.add_component(Box::new(Circle { radius: 3.0 }));
    
    println!("Running screen with components:");
    screen.run();
    
    // Vector of trait objects
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 2.5 }),
        Box::new(Rectangle { width: 8.0, height: 12.0 }),
    ];
    
    println!("\nDrawing shapes from vector:");
    for shape in &shapes {
        shape.draw();
    }
}

/// Advanced trait patterns - Blanket implementations
pub trait Display {
    fn display(&self) -> String;
}

// Blanket implementation: implement Display for any type that implements ToString
impl<T: ToString> Display for T {
    fn display(&self) -> String {
        self.to_string()
    }
}

/// Trait with associated types
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Default implementation using associated type
    fn collect_all(mut self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

pub struct Counter {
    current: i32,
    max: i32,
}

impl Counter {
    pub fn new(max: i32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

/// Demonstrates advanced trait patterns
pub fn advanced_traits_example() {
    println!("\n=== Advanced Trait Patterns ===");
    
    // Blanket implementation
    let number = 42;
    let text = "hello";
    println!("Number display: {}", number.display());
    println!("Text display: {}", text.display());
    
    // Associated types
    let mut counter = Counter::new(3);
    println!("Counter values:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }
    
    // Using collect_all
    let counter2 = Counter::new(5);
    let all_values = counter2.collect_all();
    println!("All counter values: {:?}", all_values);
}

/// Include the supertrait example from the existing module
pub fn supertrait_example() {
    println!("\n=== Supertraits ===");
    
    // We'll use a local example here since the original structs aren't public
    trait Person {
        fn name(&self) -> String;
    }
    
    trait Student: Person {
        fn university(&self) -> String;
    }
    
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }
    
    struct ExampleStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }
    
    impl Person for ExampleStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    
    impl Student for ExampleStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    
    impl Programmer for ExampleStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }
    
    impl CompSciStudent for ExampleStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }
    
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
    
    let student = ExampleStudent {
        name: "Alice Johnson".to_string(),
        university: "MIT".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "alice_codes".to_string(),
    };
    
    let greeting = comp_sci_student_greeting(&student);
    println!("Student greeting: {}", greeting);
}

/// Run all trait examples
pub fn run_all_examples() {
    println!("🦀 Traits Examples 🦀\n");
    
    basic_traits_example();
    operator_overloading_example();
    trait_objects_example();
    advanced_traits_example();
    supertrait_example();
    
    println!("\n✅ All trait examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_trait() {
        let tweet = Tweet {
            username: String::from("test_user"),
            content: String::from("test content"),
            reply: false,
            retweet: false,
        };
        
        assert_eq!(tweet.summarize(), "test_user: test content");
        assert_eq!(tweet.summarize_author(), "@test_user");
    }

    #[test]
    fn test_point_operations() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        
        let p3 = p1.clone().add(p2.clone());
        assert_eq!(p3, Point { x: 4, y: 6 });
        
        let p4 = p1.clone().add(5);
        assert_eq!(p4, Point { x: 6, y: 7 });
        
        let p5 = p1 + p2;
        assert_eq!(p5, Point { x: 4, y: 6 });
    }

    #[test]
    fn test_counter_iterator() {
        let mut counter = Counter::new(3);
        
        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), None);
        
        let counter2 = Counter::new(3);
        let values = counter2.collect_all();
        assert_eq!(values, vec![0, 1, 2]);
    }

    #[test]
    fn test_display_trait() {
        let number = 42;
        assert_eq!(number.display(), "42");
        
        let text = "hello";
        assert_eq!(text.display(), "hello");
    }

    #[test]
    fn test_point_display() {
        let point = Point { x: 5, y: 10 };
        assert_eq!(format!("{}", point), "(5, 10)");
    }
}