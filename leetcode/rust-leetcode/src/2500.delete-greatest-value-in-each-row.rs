impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            grid[i].sort();
        }
        let mut ans = 0;
        for j in 0..grid[0].len() {
            let mut m = grid[0][j];
            for i in 1..grid.len() {
                m = m.max(grid[i][j]);
            }
            ans += m;
        }
        ans
    }
}

