impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        fn backtrack(grid: &mut Vec<Vec<i32>>, gold: &mut i32, sum: i32, pos: (usize, usize), m: usize, n: usize) {
            let (x, y) = pos;
            let xd = [-1, 0, 0, 1];
            let yd = [0, -1, 1, 0];
            let v = grid[x][y];
            grid[x][y] = 0;
            let sum = sum + v;
            *gold = sum.max(*gold);
            for i in 0..4 {
                let dx = x.saturating_add_signed(xd[i]).min(m);
                let dy = y.saturating_add_signed(yd[i]).min(n);
                if grid[dx][dy] != 0 {
                    backtrack(grid, gold, sum, (dx, dy), m, n);
                }
            }
            grid[x][y] = v;
        }
        let mut gold = 0i32;
        let (m, n) = (grid.len()-1, grid[0].len()-1);
        for i in 0..=m {
            for j in 0..=n {
                if grid[i][j] != 0 {
                    backtrack(&mut grid, &mut gold, 0, (i, j), m, n);
                }
            }
        }
        gold
    }
}
