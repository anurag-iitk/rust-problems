// Merge Sorted Array
// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
// To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

 

// Example 1:

// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// Output: [1,2,2,3,5,6]
// Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
// The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
// Example 2:

// Input: nums1 = [1], m = 1, nums2 = [], n = 0
// Output: [1]
// Explanation: The arrays we are merging are [1] and [].
// The result of the merge is [1].
// Example 3:

// Input: nums1 = [0], m = 0, nums2 = [1], n = 1
// Output: [1]
// Explanation: The arrays we are merging are [] and [1].
// The result of the merge is [1].
// Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.

fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &Vec<i32>, n: usize) {
    let mut i = m as isize - 1; // Last element index in nums1 (non-zero part)
    let mut j = n as isize - 1; // Last element index in nums2
    let mut k = (m + n) as isize - 1; // Last element index for merged array

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &nums2, n);
    println!("{:?}", nums1);

    let mut nums1 = vec![1];
    let nums2 = vec![];
    let m = 1;
    let n = 0;

    merge(&mut nums1, m, &nums2, n);
    println!("{:?}", nums1);

    let mut nums1 = vec![0];
    let nums2 = vec![1];
    let m = 0;
    let n = 1;

    merge(&mut nums1, m, &nums2, n);
    println!("{:?}", nums1);
}

