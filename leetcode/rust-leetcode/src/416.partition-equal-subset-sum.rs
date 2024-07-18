impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut nums = nums.iter().map(|&n| n as usize).collect::<Vec<_>>();
        let total_sum = nums.iter().sum::<usize>();
        if total_sum & 1 == 1 {
            return false;
        }
        let sum = (total_sum/2) as usize;
        let mut dp = vec![false; sum+1];
        dp[0] = true;
        nums.sort();
        for &num in nums.iter().rev() {
            if dp[sum] {
                break;
            }
            for i in (num..=sum).rev() {
                dp[i] = dp[i] || dp[i-num];
            }
        }
        dp[sum]
    }
}
