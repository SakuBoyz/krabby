use krabby::leetcode::fibonacci::Fibonacci;

fn main() {
    println!("Fibonacci sequence:");
    let sequence = Fibonacci::fib_sequence(10);
    for (i, value) in sequence.iter().enumerate() {
        println!("fib({}) = {}", i, value);
    }
    
    println!("\nTesting fib_index:");
    let test_values = vec![0, 1, 2, 3, 5, 8, 13];
    for value in test_values {
        match Fibonacci::fib_index(value) {
            Some(index) => println!("fib_index({}) = Some({})", value, index),
            None => println!("fib_index({}) = None", value),
        }
    }
}