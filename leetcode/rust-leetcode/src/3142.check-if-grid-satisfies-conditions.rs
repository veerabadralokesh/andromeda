impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for j in 1..grid[0].len() {
            if grid[0][j] == grid[0][j-1] {
                return false;
            }
        }
        for i in 1..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != grid[i-1][j] {
                    return false;
                }
            }
        }
        true
    }
}

