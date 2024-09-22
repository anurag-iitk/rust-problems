// 1. Two Sum
// Hint
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109


fn main() {
    let nums: Vec<i32> = vec![2,7,11,15];
    let target: i32 = 9;
    let ans1: Vec<i32> = two_sum(nums.clone(), target);
    println!("{:?}", ans1);
    let ans2: Vec<i32> = two_sum_hash_map(nums, target);
    println!("{:?}", ans2);
}

// Soution 1 - using array 
// complexity - O(n^2)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &num1) in nums.iter().enumerate() {
        for (j, &num2) in nums.iter().enumerate().skip(i+1){
            if num1 + num2 == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![] 
}

// Solution 2 - using HashMap 
// complexity - O(n)
use std::collections::HashMap;

pub fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&index) = map.get(&diff) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}
