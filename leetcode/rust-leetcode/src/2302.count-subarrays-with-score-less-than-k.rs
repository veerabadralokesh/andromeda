impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut sum = 0;
        let mut i = 0;
        let mut ans = 0;
        for (j, &n) in nums.iter().enumerate() {
            sum += n as i64;
            while sum * (j - i + 1) as i64 >= k {
                sum -= nums[i] as i64;
                i += 1;
            }
            ans += (j - i + 1) as i64;
        }
        ans
    }
}

