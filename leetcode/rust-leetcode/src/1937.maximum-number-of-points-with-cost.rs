use std::cmp::max;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (points.len(), points[0].len());
        let points = points.iter().map(|row| row.iter().map(|&n| n as i64).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut dp = points[0].to_vec();
        let mut maxi = 0;
        for i in 1..m {
            let mut left_to_right = vec![0; n];
            // let mut right_to_left = vec![0; n];
            maxi = 0;
            for j in 0..n {
                maxi = max(maxi-1, dp[j]);
                left_to_right[j] = maxi;
            }
            maxi = 0;
            for j in (0..n).rev() {
                maxi = max(maxi-1, dp[j]);
                // right_to_left[j] = maxi;
                dp[j] = max(maxi, left_to_right[j]) + points[i][j];
            }
        }
        *dp.iter().max().unwrap()
    }
}

