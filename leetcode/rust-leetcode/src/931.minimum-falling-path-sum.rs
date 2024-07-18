use std::cmp::min;
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        for i in 1..n {
            for j in 0..n {
                matrix[i][j] += min(
                    matrix[i-1][j.saturating_sub(1)],
                    min(matrix[i-1][j], matrix[i-1][(n-1).min(j+1)])
                );
            }
        }
        *matrix[n-1].iter().min().unwrap()
    }
}

