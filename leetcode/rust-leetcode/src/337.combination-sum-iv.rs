impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; 1001];
        let t = target as usize;
        let mut nums = nums.iter().map(|&n| {dp[n as usize]=1; n as usize}).collect::<Vec<_>>();
        nums.sort_unstable();
        for i in 1..=t {
            for &j in nums.iter() {
                if j < i {
                    dp[i] += dp[i-j];
                } else {
                    break;
                }
            }
        }
        dp[t]
    }
}

