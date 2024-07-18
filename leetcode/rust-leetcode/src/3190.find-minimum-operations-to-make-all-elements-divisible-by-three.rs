impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|&n| n % 3 != 0).count() as _
    }
}

