impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut l, mut r, mut t, mut b) = (0, 0, 0, 0);
        let (mut row_flips, mut col_flips) = (0, 0);
        for i in 0..m {
            (l, r) = (0, n-1);
            while l < r {
                if grid[i][l] != grid[i][r] {
                    row_flips += 1;
                }
                l += 1;
                r -= 1;
            }
        }
        for j in 0..n {
            (t, b) = (0, m-1);
            while t < b {
                if grid[t][j] != grid[b][j] {
                    col_flips += 1;
                }
                t += 1;
                b -= 1;
            }
        }
        row_flips.min(col_flips)
    }
}

