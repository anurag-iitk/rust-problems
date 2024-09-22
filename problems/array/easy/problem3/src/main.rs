// 3. Remove Element
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
// Return k.

fn main() {
    let mut nums: Vec<i32> = vec![3, 2, 2, 3];
    let val: i32 = 3;
    let ans: i32 = remove_element(&mut nums, val);
    println!("{}", ans);
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;

    for i in 0..nums.len(){
        if nums[i] != val {
            nums[index] = nums[i];
            index += 1;
        }
    }
    index as i32
}
