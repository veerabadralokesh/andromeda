impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|&n| n & 1);
        nums
    }
}

/* */

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut even = 0;
        for i in 0..nums.len() {
            if nums[i] & 1 == 0 {
                nums.swap(even , i);
                even += 1;
            }
        }
        nums
    }
}

