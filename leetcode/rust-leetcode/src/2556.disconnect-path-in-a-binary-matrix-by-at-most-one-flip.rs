impl Solution {
    pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        fn check_path(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize) -> bool {
            if i == m && j == n {
                return true;
            }
            if i > m || j > n {
                return false;
            }
            if grid[i][j] == 0 {
                return false;
            }
            grid[i][j] = 0;
            check_path(grid, i+1, j, m, n) || check_path(grid, i, j+1, m, n)
        }
        let mut grid = grid;
        if !check_path(&mut grid, 0, 0, m-1, n-1) {
            return true;
        }
        grid[0][0] = 1;
        !check_path(&mut grid, 0, 0, m-1, n-1)
    }
}

