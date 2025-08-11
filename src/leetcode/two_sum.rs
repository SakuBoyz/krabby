//! Two Sum problem - Find two numbers in array that add up to target
//! 
//! Given an array of integers nums and an integer target,
//! return indices of the two numbers such that they add up to target.

use std::collections::HashMap;

pub struct TwoSum;

impl TwoSum {
    /// Brute force solution - O(n²) time, O(1) space
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
    
    /// Hash map solution - O(n) time, O(n) space
    pub fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            
            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }
            
            map.insert(num, i);
        }
        
        vec![]
    }
    
    /// One-pass hash map solution
    pub fn two_sum_one_pass(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            
            if let Some(&j) = map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            
            map.insert(num, i);
        }
        
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_brute_force() {
        assert_eq!(TwoSum::two_sum_brute_force(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(TwoSum::two_sum_brute_force(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(TwoSum::two_sum_brute_force(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_hash_map() {
        assert_eq!(TwoSum::two_sum_hash_map(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(TwoSum::two_sum_hash_map(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(TwoSum::two_sum_hash_map(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_one_pass() {
        assert_eq!(TwoSum::two_sum_one_pass(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(TwoSum::two_sum_one_pass(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(TwoSum::two_sum_one_pass(vec![3, 3], 6), vec![0, 1]);
    }
}