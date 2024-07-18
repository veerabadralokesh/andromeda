use std::cmp::max;
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![i32::MIN; n+1]; m+1];
        for i in 0..m {
            for j in 0..n {
                dp[i+1][j+1] = max(
                    max(dp[i][j+1], dp[i+1][j]),
                    max(0, dp[i][j]) + nums1[i] * nums2[j]
                )
            }
        }
        // for row in dp.iter() {
        //     println!("{:?}", row);
        // }
        dp[m][n]
    }
}

