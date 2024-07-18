use std::cmp::min;
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (m, n) = (row_sum.len(), col_sum.len());
        let mut matrix = vec![vec![0; n]; m];
        let mut x = 0;
        for i in 0..m {
            for j in 0..n {
                x = min(row_sum[i], col_sum[j]);
                matrix[i][j] = x;
                row_sum[i] -= x;
                col_sum[j] -= x;
            }
        }
        matrix
    }
}

