impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut min_max = 0i32;
        let l = nums.len();
        for i in 0..l/2 {
            min_max = std::cmp::max(min_max, nums[i] + nums[l-i-1]);
        }
        min_max
    }
}

