impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut counts = vec![vec![(0, 0); n+1]; m+1];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let x = counts[i+1][j].0 + counts[i][j+1].0 - counts[i][j].0 + if grid[i][j] == 'X' {1} else {0};
                let y = counts[i+1][j].1 + counts[i][j+1].1 - counts[i][j].1 + if grid[i][j] == 'Y' {1} else {0};
                counts[i+1][j+1] = (x, y);
                if x > 0 && x == y {
                    ans += 1;
                }
            }
        }
        ans
    }
}

