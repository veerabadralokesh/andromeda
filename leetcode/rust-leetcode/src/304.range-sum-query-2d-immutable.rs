struct NumMatrix {
    prefix_sum: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut prefix_sum = vec![vec![0; n+1]; m+1];
        for i in 0..m {
            for j in 0..n {
                prefix_sum[i+1][j+1] = matrix[i][j] + prefix_sum[i][j+1] + prefix_sum[i+1][j] - prefix_sum[i][j];
            }
        }
        NumMatrix {prefix_sum,}
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize + 1, col1 as usize + 1, row2 as usize + 1, col2 as usize + 1);
        self.prefix_sum[row2][col2] + self.prefix_sum[row1-1][col1-1] - self.prefix_sum[row1-1][col2] - self.prefix_sum[row2][col1-1]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

 