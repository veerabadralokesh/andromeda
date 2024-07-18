impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut rows = Vec::with_capacity(n);
        let mut cols = Vec::with_capacity(n);
        for i in 0..n {
            rows.push(grid[i].iter().max().unwrap());
            cols.push(0);
            for j in 0..n {
                cols[i] = cols[i].max(grid[j][i]);
            }
        }
        let mut ans = 0i32;
        for i in 0..n {
            for j in 0..n {
                ans += (rows[i].min(&cols[j]) - grid[i][j]);
            }
        }
        ans
    }
}
