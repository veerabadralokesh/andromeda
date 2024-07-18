impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for j in 0..n {
            for i in 0..m {
                rows[i] += grid[i][j];
                cols[j] += grid[i][j];
            }
            cols[j] *= 2;
        }
        let d:i32 = (m as i32 + n as i32);
        for i in 0..m {
            rows[i] *= 2;
            for j in 0..n {
                grid[i][j] = rows[i] + cols[j] - d;
            }
        }

        grid
    }
}
