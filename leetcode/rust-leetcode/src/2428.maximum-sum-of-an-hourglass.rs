impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for cr in 1..m-1 {
            let cra = cr - 1;
            let crb = cr + 1;
            let mut r1 = grid[cra][0] + grid[cra][1] + grid[cra][2];
            let mut r2 = grid[crb][0] + grid[crb][1] + grid[crb][2];
            let mut rans = r1 + r2 + grid[cr][1];
            ans = ans.max(rans);
            for cc in 2..n-1 {
                rans = rans - grid[cra][cc-2] - grid[crb][cc-2] + grid[cra][cc+1] + grid[crb][cc+1] - grid[cr][cc-1] + grid[cr][cc];
                ans = ans.max(rans);
            }
        }
        ans
    }
}

