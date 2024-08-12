use std::collections::VecDeque;
static directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut g = grid;
        fn bfs(
            g: &Vec<Vec<i32>>,
            set: &mut Vec<Vec<bool>>,
            i: usize,
            j: usize,
            m: usize,
            n: usize
        ) {
            let mut q = VecDeque::new();
            q.push_back((i, j));
            set[i][j] = true;
            let (mut di, mut dj) = (0, 0);
            while !q.is_empty() {
                for _ in 0..q.len() {
                    if let Some((i, j)) = q.pop_front() {
                        for dir in directions.iter() {
                            di = i.saturating_add_signed(dir.0).min(m);
                            dj = j.saturating_add_signed(dir.1).min(n);
                            if g[di][dj] == 1 && !set[di][dj] {
                                set[di][dj] = true;
                                q.push_back((di, dj));
                            }
                        }
                    }
                }
            }
        }
        fn is_disconnected(g: &Vec<Vec<i32>>, m: usize, n: usize) -> bool {
            let mut num_islands = 0;
            let mut set = vec![vec![false; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if g[i][j] == 1 && !set[i][j] {
                        bfs(&g, &mut set, i, j, m-1, n-1);
                        num_islands += 1;
                    }
                }
            }
            num_islands != 1
        }
        if is_disconnected(&g, m, n) {
            return 0;
        }
        for i in 0..m {
            for j in 0..n {
                if g[i][j] == 1 {
                    g[i][j] = 0;
                    if is_disconnected(&g, m, n) {
                        return 1;
                    }
                    g[i][j] = 1;
                }
            }
        }
        2
    }
}

