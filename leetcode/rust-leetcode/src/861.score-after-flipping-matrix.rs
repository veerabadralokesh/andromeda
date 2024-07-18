impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            if grid[i][0] == 0 {
                for j in 0..n {
                    grid[i][j] = 1-grid[i][j];
                }
            }
        }
        let mut ans = (m * (1 << (n-1))) as i32;
        for j in 1..n {
            let mut ones = 0i32;
            for i in 0..m {ones += grid[i][j];}
            ones = ones.max((m as i32)-ones);
            ans += ones * (1 << (n-1-j) as i32);
        }
        ans
    }
}
