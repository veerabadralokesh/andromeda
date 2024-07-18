impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0i32;
        let n = mat.len();
        for i in 0..n {
            if i == n-i-1 {
                ans += mat[i][i];
                continue;
            }
            ans += mat[i][i] + mat[i][n-i-1];
        }
        ans
    }
}
