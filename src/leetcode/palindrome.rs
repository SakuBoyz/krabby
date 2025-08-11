//! Palindrome checker - Check if a string or number is a palindrome
//! 
//! A palindrome reads the same forward and backward.

pub struct Palindrome;

impl Palindrome {
    /// Check if a string is a palindrome (case insensitive, ignoring spaces)
    pub fn is_palindrome_string(s: &str) -> bool {
        let cleaned: String = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        let reversed: String = cleaned.chars().rev().collect();
        cleaned == reversed
    }
    
    /// Check if a string is a palindrome using two pointers
    pub fn is_palindrome_two_pointers(s: &str) -> bool {
        let chars: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        let mut left = 0;
        let mut right = chars.len();
        
        while left < right {
            right -= 1;
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
        }
        
        true
    }
    
    /// Check if a number is a palindrome
    pub fn is_palindrome_number(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        
        let s = x.to_string();
        let reversed: String = s.chars().rev().collect();
        s == reversed
    }
    
    /// Check if a number is a palindrome without converting to string
    pub fn is_palindrome_number_math(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        
        let mut reversed = 0;
        
        while x > reversed {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }
        
        // For even length: x == reversed
        // For odd length: x == reversed / 10
        x == reversed || x == reversed / 10
    }
    
    /// Find the longest palindromic substring
    pub fn longest_palindromic_substring(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }
        
        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 1;
        
        for i in 0..chars.len() {
            // Check for odd length palindromes
            let len1 = Self::expand_around_center(&chars, i, i);
            // Check for even length palindromes
            let len2 = Self::expand_around_center(&chars, i, i + 1);
            
            let len = len1.max(len2);
            if len > max_len {
                max_len = len;
                start = i - (len - 1) / 2;
            }
        }
        
        chars[start..start + max_len].iter().collect()
    }
    
    fn expand_around_center(chars: &[char], mut left: usize, mut right: usize) -> usize {
        while right < chars.len() && chars[left] == chars[right] {
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
        
        if left == 0 && right < chars.len() && chars[left] == chars[right] {
            right + 1
        } else {
            right - left - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_string() {
        assert!(Palindrome::is_palindrome_string("A man a plan a canal Panama"));
        assert!(Palindrome::is_palindrome_string("race a car") == false);
        assert!(Palindrome::is_palindrome_string(""));
        assert!(Palindrome::is_palindrome_string("a"));
        assert!(Palindrome::is_palindrome_string("Madam"));
        assert!(Palindrome::is_palindrome_string("Was it a car or a cat I saw"));
    }

    #[test]
    fn test_is_palindrome_two_pointers() {
        assert!(Palindrome::is_palindrome_two_pointers("A man a plan a canal Panama"));
        assert!(Palindrome::is_palindrome_two_pointers("race a car") == false);
        assert!(Palindrome::is_palindrome_two_pointers(""));
        assert!(Palindrome::is_palindrome_two_pointers("a"));
    }

    #[test]
    fn test_is_palindrome_number() {
        assert!(Palindrome::is_palindrome_number(121));
        assert!(Palindrome::is_palindrome_number(-121) == false);
        assert!(Palindrome::is_palindrome_number(10) == false);
        assert!(Palindrome::is_palindrome_number(0));
        assert!(Palindrome::is_palindrome_number(1221));
    }

    #[test]
    fn test_is_palindrome_number_math() {
        assert!(Palindrome::is_palindrome_number_math(121));
        assert!(Palindrome::is_palindrome_number_math(-121) == false);
        assert!(Palindrome::is_palindrome_number_math(10) == false);
        assert!(Palindrome::is_palindrome_number_math(0));
        assert!(Palindrome::is_palindrome_number_math(1221));
    }

    #[test]
    fn test_longest_palindromic_substring() {
        assert_eq!(Palindrome::longest_palindromic_substring("babad"), "bab");
        assert_eq!(Palindrome::longest_palindromic_substring("cbbd"), "bb");
        assert_eq!(Palindrome::longest_palindromic_substring("a"), "a");
        assert_eq!(Palindrome::longest_palindromic_substring("ac"), "a");
        assert_eq!(Palindrome::longest_palindromic_substring(""), "");
    }
}