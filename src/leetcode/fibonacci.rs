//! Fibonacci sequence implementations
//! 
//! Different ways to calculate Fibonacci numbers with varying time/space complexity

pub struct Fibonacci;

impl Fibonacci {
    /// Recursive implementation - O(2^n) time, O(n) space
    pub fn fib_recursive(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => Self::fib_recursive(n - 1) + Self::fib_recursive(n - 2),
        }
    }
    
    /// Memoized recursive implementation - O(n) time, O(n) space
    pub fn fib_memoized(n: u32) -> u64 {
        use std::collections::HashMap;
        
        fn fib_helper(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
            if let Some(&value) = memo.get(&n) {
                return value;
            }
            
            let result = match n {
                0 => 0,
                1 => 1,
                _ => fib_helper(n - 1, memo) + fib_helper(n - 2, memo),
            };
            
            memo.insert(n, result);
            result
        }
        
        let mut memo = HashMap::new();
        fib_helper(n, &mut memo)
    }
    
    /// Iterative implementation - O(n) time, O(1) space
    pub fn fib_iterative(n: u32) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        let mut prev = 0;
        let mut curr = 1;
        
        for _ in 2..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        
        curr
    }
    
    /// Generate first n Fibonacci numbers
    pub fn fib_sequence(n: u32) -> Vec<u64> {
        let mut sequence = Vec::new();
        
        for i in 0..n {
            sequence.push(Self::fib_iterative(i));
        }
        
        sequence
    }
    
    /// Check if a number is a Fibonacci number
    pub fn is_fibonacci(num: u64) -> bool {
        let mut a = 0;
        let mut b = 1;
        
        if num == 0 || num == 1 {
            return true;
        }
        
        while b < num {
            let next = a + b;
            a = b;
            b = next;
        }
        
        b == num
    }
    
    /// Find the index of a Fibonacci number (returns None if not found)
    pub fn fib_index(num: u64) -> Option<u32> {
        if num == 0 {
            return Some(0);
        }
        if num == 1 {
            return Some(1);
        }
        
        let mut a = 0;
        let mut b = 1;
        let mut index = 1;
        
        while b < num {
            let next = a + b;
            a = b;
            b = next;
            index += 1;
        }
        
        if b == num {
            Some(index)
        } else {
            None
        }
    }
    
    /// Matrix exponentiation method - O(log n) time
    pub fn fib_matrix(n: u32) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        let base = [[1, 1], [1, 0]];
        let result = Self::matrix_power(base, n - 1);
        result[0][0]
    }
    
    fn matrix_multiply(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
        [
            [a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1]],
            [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1]],
        ]
    }
    
    fn matrix_power(mut base: [[u64; 2]; 2], mut exp: u32) -> [[u64; 2]; 2] {
        let mut result = [[1, 0], [0, 1]]; // Identity matrix
        
        while exp > 0 {
            if exp % 2 == 1 {
                result = Self::matrix_multiply(result, base);
            }
            base = Self::matrix_multiply(base, base);
            exp /= 2;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_recursive() {
        assert_eq!(Fibonacci::fib_recursive(0), 0);
        assert_eq!(Fibonacci::fib_recursive(1), 1);
        assert_eq!(Fibonacci::fib_recursive(2), 1);
        assert_eq!(Fibonacci::fib_recursive(3), 2);
        assert_eq!(Fibonacci::fib_recursive(4), 3);
        assert_eq!(Fibonacci::fib_recursive(5), 5);
        assert_eq!(Fibonacci::fib_recursive(6), 8);
        assert_eq!(Fibonacci::fib_recursive(7), 13);
        // Don't test large numbers with recursive as it's exponential
    }

    #[test]
    fn test_fib_memoized() {
        assert_eq!(Fibonacci::fib_memoized(0), 0);
        assert_eq!(Fibonacci::fib_memoized(1), 1);
        assert_eq!(Fibonacci::fib_memoized(10), 55);
        assert_eq!(Fibonacci::fib_memoized(20), 6765);
        assert_eq!(Fibonacci::fib_memoized(30), 832040);
    }

    #[test]
    fn test_fib_iterative() {
        assert_eq!(Fibonacci::fib_iterative(0), 0);
        assert_eq!(Fibonacci::fib_iterative(1), 1);
        assert_eq!(Fibonacci::fib_iterative(10), 55);
        assert_eq!(Fibonacci::fib_iterative(20), 6765);
        assert_eq!(Fibonacci::fib_iterative(30), 832040);
        assert_eq!(Fibonacci::fib_iterative(40), 102334155);
    }

    #[test]
    fn test_fib_sequence() {
        assert_eq!(Fibonacci::fib_sequence(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(Fibonacci::fib_sequence(5), vec![0, 1, 1, 2, 3]);
        assert_eq!(Fibonacci::fib_sequence(1), vec![0]);
        assert_eq!(Fibonacci::fib_sequence(0), vec![]);
    }

    #[test]
    fn test_is_fibonacci() {
        assert!(Fibonacci::is_fibonacci(0));
        assert!(Fibonacci::is_fibonacci(1));
        assert!(Fibonacci::is_fibonacci(2));
        assert!(Fibonacci::is_fibonacci(3));
        assert!(Fibonacci::is_fibonacci(5));
        assert!(Fibonacci::is_fibonacci(8));
        assert!(Fibonacci::is_fibonacci(13));
        assert!(Fibonacci::is_fibonacci(21));
        
        assert!(!Fibonacci::is_fibonacci(4));
        assert!(!Fibonacci::is_fibonacci(6));
        assert!(!Fibonacci::is_fibonacci(7));
        assert!(!Fibonacci::is_fibonacci(9));
        assert!(!Fibonacci::is_fibonacci(10));
    }

    #[test]
    fn test_fib_index() {
        assert_eq!(Fibonacci::fib_index(0), Some(0));
        assert_eq!(Fibonacci::fib_index(1), Some(1)); // Returns first occurrence at index 1
        assert_eq!(Fibonacci::fib_index(2), Some(3));  // 2 is at index 3
        assert_eq!(Fibonacci::fib_index(3), Some(4));  // 3 is at index 4
        assert_eq!(Fibonacci::fib_index(5), Some(5));  // 5 is at index 5
        assert_eq!(Fibonacci::fib_index(8), Some(6));  // 8 is at index 6
        assert_eq!(Fibonacci::fib_index(13), Some(7)); // 13 is at index 7
        
        assert_eq!(Fibonacci::fib_index(4), None);
        assert_eq!(Fibonacci::fib_index(6), None);
        assert_eq!(Fibonacci::fib_index(7), None);
    }

    #[test]
    fn test_fib_matrix() {
        assert_eq!(Fibonacci::fib_matrix(0), 0);
        assert_eq!(Fibonacci::fib_matrix(1), 1);
        assert_eq!(Fibonacci::fib_matrix(10), 55);
        assert_eq!(Fibonacci::fib_matrix(20), 6765);
        assert_eq!(Fibonacci::fib_matrix(30), 832040);
    }

    #[test]
    fn test_all_methods_consistency() {
        // Test that all methods give the same results for small values
        for i in 0..=15 {
            let recursive = Fibonacci::fib_recursive(i);
            let memoized = Fibonacci::fib_memoized(i);
            let iterative = Fibonacci::fib_iterative(i);
            let matrix = Fibonacci::fib_matrix(i);
            
            assert_eq!(recursive, memoized, "Mismatch at fib({})", i);
            assert_eq!(recursive, iterative, "Mismatch at fib({})", i);
            assert_eq!(recursive, matrix, "Mismatch at fib({})", i);
        }
    }
}