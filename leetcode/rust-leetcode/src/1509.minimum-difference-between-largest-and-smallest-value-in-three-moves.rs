impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 5 {
            return 0;
        }
        let mut ans = i32::MAX;
        nums.sort();
        for i in 0..4 {
            ans = ans.min(nums[nums.len() - 4 + i] - nums[i]);
        }
        ans
    }
}

