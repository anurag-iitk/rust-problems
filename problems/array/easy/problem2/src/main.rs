
// 2. Remove Duplicates from Sorted Array
// Hint
// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
// Return k.

// Example 1:

// Input: nums = [1,1,2]
// Output: 2, nums = [1,2,_]
// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
// Example 2:

// Input: nums = [0,0,1,1,1,2,2,3,3,4]
// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
 

// Constraints:

// 1 <= nums.length <= 3 * 104
// -100 <= nums[i] <= 100
// nums is sorted in non-decreasing order.

fn main() {
    let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
    let ans: i32 = remove_duplicates(&mut nums);
    println!("{}", ans);
}

// Approach 1 - Using Two Pointer Technique
// Complexity - O(n)
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty(){
        return 0;
    }

    let mut i = 0;

    for j in 1..nums.len(){
        if nums[j] != nums[i]{
            i += 1;
            nums[i] = nums[j];
        }
    }
    (i + 1) as i32
}

// Approach 2 - Using Two Pointer with Unique Count
// Complexity - O(n)
pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty(){
        return 0;
    }

    let mut unique_count = 1;

    for i in 1..nums.len(){
        if nums[i] != nums[unique_count - 1] {
            nums[unique_count] = nums[i];
            unique_count += 1;
        }
    }
    unique_count as i32
}

// Approach 3 - Using Retail Method
// Complexity - O(n)
pub fn remove_duplicates_3(nums: &mut Vec<i32>) -> i32 {
    let mut last_seen: Option<i32> = None;
    nums.retain(|&x| {
        if Some(x) != last_seen {
            last_seen = Some(x);
            true
        } else {
            false
        }
    });

    nums.len() as i32;
}

// Approach 4 - Using HashSet
// Complexity - O(n)
use std::collections::HashSet;

pub fn remove_duplicates_4(nums: &mut Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    nums.retain(|&x| set.insert(x));

    nums.len() as i32
}

