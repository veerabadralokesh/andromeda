use std::cmp::{min,max};
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut min_i, mut max_i, mut min_j, mut max_j) = (m-1, 0, n-1, 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    min_i = min(min_i, i);
                    max_i = max(max_i, i);
                    min_j = min(min_j, j);
                    max_j = max(max_j, j);
                }
            }
        }
        ((max_i - min_i + 1) * (max_j - min_j + 1)) as _
    }
}

