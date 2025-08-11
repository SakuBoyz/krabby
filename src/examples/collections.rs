//! Collections examples
//! 
//! This module demonstrates Rust's common collections:
//! - Vectors (Vec<T>)
//! - Strings
//! - Hash Maps (HashMap<K, V>)
//! - Other useful collections

use std::collections::HashMap;

/// Demonstrates vector usage and operations
pub fn vector_example() {
    println!("=== Vectors ===");
    
    // Creating vectors
    let v: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v);
    
    let v = vec![1, 2, 3];
    println!("Vector with macro: {:?}", v);
    
    // Adding elements
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("After pushing elements: {:?}", v);
    
    // Reading elements
    let v = vec![1, 2, 3, 4, 5];
    
    // Method 1: Using indexing (will panic if index doesn't exist)
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    // Method 2: Using get method (returns Option)
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    let tenth = v.get(10);
    match tenth {
        Some(tenth) => println!("The tenth element is {}", tenth),
        None => println!("There is no tenth element."),
    }
    
    // Iterating over vectors
    println!("Iterating over vector:");
    for i in &v {
        println!("{}", i);
    }
    
    // Iterating with mutable references
    let mut v = vec![100, 32, 57];
    println!("Before mutation: {:?}", v);
    for i in &mut v {
        *i += 50; // Dereference to get the value
    }
    println!("After mutation: {:?}", v);
    
    // Using enums to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
        }
    }
    
    // Vector methods
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", v);
    println!("Length: {}", v.len());
    println!("Is empty: {}", v.is_empty());
    
    // Pop removes and returns the last element
    if let Some(last) = v.pop() {
        println!("Popped element: {}", last);
    }
    println!("After pop: {:?}", v);
    
    // Insert at specific position
    v.insert(1, 10);
    println!("After insert at index 1: {:?}", v);
    
    // Remove at specific position
    let removed = v.remove(1);
    println!("Removed element: {}, vector now: {:?}", removed, v);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/// Demonstrates string operations
pub fn string_example() {
    println!("\n=== Strings ===");
    
    // Creating strings
    let s = String::new();
    println!("Empty string: '{}'", s);
    
    let data = "initial contents";
    let s = data.to_string();
    println!("From to_string(): '{}'", s);
    
    let s = String::from("initial contents");
    println!("From String::from(): '{}'", s);
    
    // Updating strings
    let mut s = String::from("foo");
    s.push_str("bar"); // Appends a string slice
    println!("After push_str: '{}'", s);
    
    s.push('!'); // Appends a single character
    println!("After push: '{}'", s);
    
    // Concatenation with +
    let _s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = _s1 + &s2; // s1 has been moved here and can no longer be used
    println!("Concatenated: '{}'", s3);
    // Note: s1 is no longer valid here, but s2 is still valid
    println!("s2 is still valid: '{}'", s2);
    
    // Concatenation with format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted: '{}'", s);
    // All original strings are still valid here
    
    // Indexing into strings (not allowed in Rust)
    let s1 = String::from("hello");
    // let h = s1[0]; // This would cause a compile error!
    
    // String slicing (be careful with UTF-8)
    let hello = "Здравствуйте"; // Russian "Hello"
    let s = &hello[0..4]; // This works because each Cyrillic character is 2 bytes
    println!("String slice: '{}'", s);
    
    // Iterating over strings
    println!("Characters in 'Зд':");
    for c in "Зд".chars() {
        println!("{}", c);
    }
    
    println!("Bytes in 'Зд':");
    for b in "Зд".bytes() {
        println!("{}", b);
    }
    
    // Common string methods
    let s = String::from("  Hello, world!  ");
    println!("Original: '{}'", s);
    println!("Trimmed: '{}'", s.trim());
    println!("Length: {}", s.len());
    println!("Contains 'world': {}", s.contains("world"));
    println!("Starts with 'Hello': {}", s.trim().starts_with("Hello"));
    println!("Ends with '!': {}", s.trim().ends_with("!"));
    
    let words: Vec<&str> = s.trim().split(", ").collect();
    println!("Split words: {:?}", words);
}

/// Demonstrates HashMap usage
pub fn hashmap_example() {
    println!("\n=== Hash Maps ===");
    
    // Creating hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Initial scores: {:?}", scores);
    
    // Creating from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores from vectors: {:?}", scores);
    
    // Hash maps and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    println!("Map with owned strings: {:?}", map);
    
    // Accessing values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {}", score);
    
    // Iterating over key-value pairs
    println!("All scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Updating values
    let mut scores = HashMap::new();
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("After overwriting: {:?}", scores);
    
    // Only inserting if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("After conditional inserts: {:?}", scores);
    
    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word count: {:?}", map);
}

/// Demonstrates other useful collections
pub fn other_collections_example() {
    println!("\n=== Other Collections ===");
    
    // VecDeque (double-ended queue)
    use std::collections::VecDeque;
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("VecDeque: {:?}", deque);
    
    if let Some(front) = deque.pop_front() {
        println!("Popped from front: {}", front);
    }
    
    if let Some(back) = deque.pop_back() {
        println!("Popped from back: {}", back);
    }
    
    println!("Remaining VecDeque: {:?}", deque);
    
    // HashSet
    use std::collections::HashSet;
    let mut books = HashSet::new();
    
    books.insert("A Dance With Dragons");
    books.insert("To Kill a Mockingbird");
    books.insert("The Odyssey");
    books.insert("The Great Gatsby");
    
    if !books.contains("The Winds of Winter") {
        println!("We don't have The Winds of Winter yet.");
    }
    
    books.insert("The Winds of Winter");
    
    if books.contains("The Winds of Winter") {
        println!("We now have The Winds of Winter!");
    }
    
    println!("Number of books: {}", books.len());
    
    // Set operations
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
    
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("Intersection: {:?}", intersection);
    
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("Union: {:?}", union);
    
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("Difference (set1 - set2): {:?}", difference);
    
    // BTreeMap (ordered map)
    use std::collections::BTreeMap;
    let mut count = BTreeMap::new();
    let text = "hello world wonderful world hello";
    
    for word in text.split_whitespace() {
        *count.entry(word).or_insert(0) += 1;
    }
    
    println!("Ordered word count:");
    for (word, count) in &count {
        println!("{}: {}", word, count);
    }
}

/// Demonstrates working with collections and iterators
pub fn iterator_example() {
    println!("\n=== Iterators with Collections ===");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // Collect into new vector
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", v);
    println!("Doubled: {:?}", doubled);
    
    // Filter and collect
    let even: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even);
    
    // Fold (reduce)
    let sum: i32 = v.iter().fold(0, |acc, &x| acc + x);
    println!("Sum using fold: {}", sum);
    
    // Find
    let found = v.iter().find(|&&x| x > 3);
    match found {
        Some(value) => println!("First value > 3: {}", value),
        None => println!("No value > 3 found"),
    }
    
    // Any and all
    let has_even = v.iter().any(|&x| x % 2 == 0);
    let all_positive = v.iter().all(|&x| x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);
    
    // Working with strings
    let text = "hello world wonderful programming";
    let words: Vec<&str> = text.split_whitespace().collect();
    let long_words: Vec<&str> = words.iter()
        .filter(|&&word| word.len() > 5)
        .cloned()
        .collect();
    
    println!("All words: {:?}", words);
    println!("Long words (>5 chars): {:?}", long_words);
    
    // Chaining operations
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // Keep even numbers
        .map(|&x| x * x)            // Square them
        .filter(|&x| x > 10)        // Keep those > 10
        .collect();
    
    println!("Original numbers: {:?}", numbers);
    println!("Even → squared → >10: {:?}", result);
}

/// Run all collection examples
pub fn run_all_examples() {
    println!("🦀 Collections Examples 🦀\n");
    
    vector_example();
    string_example();
    hashmap_example();
    other_collections_example();
    iterator_example();
    
    println!("\n✅ All collection examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v, vec![1, 2, 3, 4]);
        
        let popped = v.pop();
        assert_eq!(popped, Some(4));
        assert_eq!(v, vec![1, 2, 3]);
        
        v.insert(1, 10);
        assert_eq!(v, vec![1, 10, 2, 3]);
        
        let removed = v.remove(1);
        assert_eq!(removed, 10);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_string_operations() {
        let mut s = String::from("hello");
        s.push_str(", world");
        s.push('!');
        assert_eq!(s, "hello, world!");
        
        assert!(s.contains("world"));
        assert!(s.starts_with("hello"));
        assert!(s.ends_with("!"));
        
        let trimmed = "  spaces  ".trim();
        assert_eq!(trimmed, "spaces");
    }

    #[test]
    fn test_hashmap_operations() {
        let mut scores = HashMap::new();
        scores.insert("Blue", 10);
        scores.insert("Yellow", 50);
        
        assert_eq!(scores.get("Blue"), Some(&10));
        assert_eq!(scores.get("Red"), None);
        
        scores.entry("Red").or_insert(25);
        assert_eq!(scores.get("Red"), Some(&25));
        
        // Count occurrences
        let text = "hello hello world";
        let mut word_count = HashMap::new();
        for word in text.split_whitespace() {
            *word_count.entry(word).or_insert(0) += 1;
        }
        
        assert_eq!(word_count.get("hello"), Some(&2));
        assert_eq!(word_count.get("world"), Some(&1));
    }

    #[test]
    fn test_iterator_operations() {
        let v = vec![1, 2, 3, 4, 5];
        
        let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        
        let even: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(even, vec![&2, &4]);
        
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 15);
        
        let found = v.iter().find(|&&x| x > 3);
        assert_eq!(found, Some(&4));
        
        let has_even = v.iter().any(|&x| x % 2 == 0);
        assert!(has_even);
        
        let all_positive = v.iter().all(|&x| x > 0);
        assert!(all_positive);
    }

    #[test]
    fn test_spreadsheet_cell() {
        let cells = vec![
            SpreadsheetCell::Int(42),
            SpreadsheetCell::Float(3.14),
            SpreadsheetCell::Text(String::from("hello")),
        ];
        
        assert_eq!(cells.len(), 3);
        
        match &cells[0] {
            SpreadsheetCell::Int(i) => assert_eq!(*i, 42),
            _ => panic!("Expected Int"),
        }
        
        match &cells[1] {
            SpreadsheetCell::Float(f) => assert_eq!(*f, 3.14),
            _ => panic!("Expected Float"),
        }
        
        match &cells[2] {
            SpreadsheetCell::Text(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected Text"),
        }
    }
}