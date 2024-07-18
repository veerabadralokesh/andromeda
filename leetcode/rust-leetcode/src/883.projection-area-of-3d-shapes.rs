impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    ans += 1;
                    rows[i] = rows[i].max(grid[i][j]);
                    cols[j] = cols[j].max(grid[i][j]);
                }
            }
            ans += rows[i];
        }
        ans += cols.iter().sum::<i32>();
        ans
    }
}
