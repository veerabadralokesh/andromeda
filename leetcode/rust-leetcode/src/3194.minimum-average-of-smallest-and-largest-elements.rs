impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        let mut ans = f64::MAX;
        nums.sort();
        let (mut l, mut r) = (0, nums.len()-1);
        while l < r {
            ans = ans.min((nums[l] as f64 + nums[r] as f64)/2.0);
            l += 1;
            r -= 1;
        }
        ans
    }
}

