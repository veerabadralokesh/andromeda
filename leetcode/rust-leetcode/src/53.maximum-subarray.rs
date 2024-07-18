use std::cmp::max;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0i32;
        let mut maxsum = -100000i32;
        for n in nums {
            sum = max(n, sum + n);
            maxsum = max(sum, maxsum);
        }
        maxsum
    }
}
