use std::collections::*;

type Matrix = Vec<Vec<i32>>;

static directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn count_sub_islands(grid1: Matrix, mut grid2: Matrix) -> i32 {
        let (m, n) = (grid1.len(), grid1[0].len());

        fn bfs(g1: &Matrix, g2: &mut Matrix, start: (usize, usize), m: usize, n: usize) -> i32 {
            let mut q = VecDeque::new();
            q.push_back(start);
            let (mut di, mut dj) = (0, 0);
            let mut flag = g1[start.0][start.1] == 1;
            g2[start.0][start.1] = 2;
            while let Some((i, j)) = q.pop_front() {
                for dir in directions.iter() {
                    di = (i.saturating_add_signed(dir.0)).min(m);
                    dj = (j.saturating_add_signed(dir.1)).min(n);
                    if g2[di][dj] == 1 {
                        g2[di][dj] = 2;
                        q.push_back((di, dj));
                        if flag && g1[di][dj] != 1 {
                            flag = false;
                        }
                    }
                }
            }
            flag as _
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    ans += bfs(&grid1, &mut grid2, (i, j), m-1, n-1);
                }
            }
        }
        ans
    }
}

