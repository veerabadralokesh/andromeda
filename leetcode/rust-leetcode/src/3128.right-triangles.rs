impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..m {
            if rows[i] < 2 {
                continue;
            }
            for j in 0..n {
                if cols[j] < 2 {
                    continue;
                }
                if grid[i][j] == 1 {
                    ans += (rows[i]-1) * (cols[j]-1);
                }
            }
        }
        ans
    }
}

