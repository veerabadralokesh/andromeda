impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = (k+1) as usize;
        let mut dp = vec![0; k];
        let n = arr.len();
        let mut cmax = 0;
        let mut end = 0;
        for start in (0..n).rev() {
            cmax = 0;
            end = n.min(k-1+start);
            for i in start..end {
                cmax = cmax.max(arr[i]);
                dp[start % k] = dp[start % k].max(dp[(i+1) % k] + (cmax * ((i-start+1) as i32)));
            }
        }
        dp[0]
    }
}
