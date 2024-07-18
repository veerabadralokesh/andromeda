impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let k = (k as usize) % (m * n);
        if k == 0 {
            return grid;
        }
        // let mn_k = m * n + k;
        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mij = k + (i * n) + j;
                let new_i = (mij / n) % m;
                let new_j = mij % n;
                ans[new_i][new_j] = grid[i][j];
            }
        }
        ans
    }
}

