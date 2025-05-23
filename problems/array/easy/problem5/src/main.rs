// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer.
// The digits are ordered from most significant to least significant in left-to-right order. 
// The large integer does not contain any leading 0's.
// Increment the large integer by one and return the resulting array of digits.

 

// Example 1:

// Input: digits = [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.
// Incrementing by one gives 123 + 1 = 124.
// Thus, the result should be [1,2,4].
// Example 2:

// Input: digits = [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.
// Incrementing by one gives 4321 + 1 = 4322.
// Thus, the result should be [4,3,2,2].
// Example 3:

// Input: digits = [9]
// Output: [1,0]
// Explanation: The array represents the integer 9.
// Incrementing by one gives 9 + 1 = 10.
// Thus, the result should be [1,0].


fn plus_one(digits: &mut Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
        let sum = digits[i] + carry;
        if sum == 10 {
            digits[i] = 0;
            carry = 1;
        } else {
            digits[i] = sum;
            carry = 0;
            break;
        }
    }
    if carry == 1 {
        digits.insert(0, 1);
    }
    digits.to_vec()
}

fn main() {
    let mut digits1 = vec![1, 2, 3];
    let mut digits2 = vec![4, 3, 2, 1];
    let mut digits3 = vec![9];

    println!("{:?}", plus_one(&mut digits1));
    println!("{:?}", plus_one(&mut digits2));
    println!("{:?}", plus_one(&mut digits3));
}
