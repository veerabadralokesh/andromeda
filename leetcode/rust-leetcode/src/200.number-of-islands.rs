
use std::collections::VecDeque;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut island_num = 0;
        let mut grid = grid.iter().map(|r| r.iter().map(|&rc| ((rc as u8) - b'0')).collect::<Vec<_>>()).collect::<Vec<_>>();
        fn bfs(grid: &mut Vec<Vec<u8>>, rc: (usize, usize)) {
            let mut q = VecDeque::new();
            q.push_back(rc);
            let dr = [-1, 0, 0, 1];
            let dc = [0, -1, 1, 0];
            grid[rc.0][rc.1] = 2;
            while !q.is_empty() {
                match(q.pop_front()) {
                    Some((r, c)) => {
                        for i in 0..4 {
                            let rn = r.saturating_add_signed(dr[i]);
                            let cn = c.saturating_add_signed(dc[i]);
                            if rn < grid.len() && cn < grid[0].len() && grid[rn][cn] == 1 {
                                q.push_back((rn, cn));
                                grid[rn][cn] = 2;
                            }
                        }
                    },
                    None => {}
                }
            }
        }
        for i in 0..m {
            for j in 0.. n {
                if grid[i][j] == 1 {
                    bfs(&mut grid, (i, j));
                    island_num += 1;
                }
            }
        }
        island_num
    }
}
