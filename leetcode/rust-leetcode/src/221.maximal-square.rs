use std::cmp::min;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_square = 0;
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut mat = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    mat[i][j] = 1;
                    if i > 0 && j > 0 {
                        mat[i][j] = 1 + min(
                            mat[i-1][j-1],
                            min(mat[i-1][j], mat[i][j-1])
                        );
                        max_square = max_square.max(mat[i][j]);
                    } else {
                        mat[i][j] = 1;
                        max_square = max_square.max(1);
                    }
                }
            }
        }
        max_square * max_square
    }
}

