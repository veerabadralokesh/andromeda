impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut cols = vec![-1; n];
        for i in 0..m {
            for j in 0..n {
                cols[j] = cols[j].max(matrix[i][j]);
            }
        }
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == -1 {
                    matrix[i][j] = cols[j];
                }
            }
        }
        matrix
    }
}

