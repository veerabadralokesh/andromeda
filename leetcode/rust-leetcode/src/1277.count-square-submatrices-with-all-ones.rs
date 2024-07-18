impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans:i32 = 0;
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    ans += matrix[i][j];
                } else if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i-1][j-1].min(matrix[i-1][j]).min(matrix[i][j-1]);
                    ans += matrix[i][j];
                }
            }
        }
        ans
        // matrix.into_iter().map(|r| r.iter().sum::<i32>()).collect::<Vec<i32>>().iter().sum()
    }
}
