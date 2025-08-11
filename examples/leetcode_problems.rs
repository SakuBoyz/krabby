use krabby::leetcode::{two_sum::TwoSum, palindrome::Palindrome, fibonacci::Fibonacci, reverse_int::ReverseInt};

fn main() {
    println!("🦀 LeetCode Practice Problems 🦀\n");
    
    // Two Sum problem
    println!("=== Two Sum ===");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = TwoSum::two_sum_hash_map(nums.clone(), target);
    println!("Array: {:?}, Target: {}, Indices: {:?}", nums, target, result);
    
    // Palindrome problems
    println!("\n=== Palindrome ===");
    let test_string = "A man a plan a canal Panama";
    println!("Is '{}' a palindrome? {}", test_string, Palindrome::is_palindrome_string(test_string));
    
    let test_number = 12321;
    println!("Is {} a palindrome? {}", test_number, Palindrome::is_palindrome_number(test_number));
    
    let test_string2 = "babad";
    println!("Longest palindromic substring in '{}': '{}'", 
             test_string2, 
             Palindrome::longest_palindromic_substring(test_string2));
    
    // Fibonacci sequence
    println!("\n=== Fibonacci ===");
    let n = 10;
    println!("Fibonacci({}) = {}", n, Fibonacci::fib_iterative(n));
    println!("First {} Fibonacci numbers: {:?}", n, Fibonacci::fib_sequence(n));
    println!("Is 21 a Fibonacci number? {}", Fibonacci::is_fibonacci(21));
    println!("Is 22 a Fibonacci number? {}", Fibonacci::is_fibonacci(22));
    
    // Reverse Integer
    println!("\n=== Reverse Integer ===");
    let test_numbers = vec![123, -123, 120, 1534236469];
    for num in test_numbers {
        println!("Reverse of {} is {}", num, ReverseInt::reverse(num));
    }
    
    println!("\n✅ All LeetCode examples completed!");
}