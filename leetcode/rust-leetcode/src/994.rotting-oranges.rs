use std::collections::VecDeque;
use std::cmp::{min,max};
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    grid[i][j] -= 2;
                } else {
                    grid[i][j] = i32::MAX;
                }
            }
        }
        // println!("{:?}", grid);
        fn bfs(grid: &mut Vec<Vec<i32>>, pos: (usize, usize), m: usize, n: usize) {
            let mut q = VecDeque::new();
            q.push_back((pos, 0i32));
            let xd = [-1, 0, 0, 1];
            let yd = [0, -1, 1, 0];
            while let Some((p, t)) = q.pop_front() {
                let new_t = t + 1;
                for i in 0..4 {
                    let dx = p.0.saturating_add_signed(xd[i]);
                    let dy = p.1.saturating_add_signed(yd[i]);
                    if dx >= m || dy >= n || grid[dx][dy] == -2 || grid[dx][dy] == 0 {
                        continue;
                    }
                    if new_t < grid[dx][dy] {
                        grid[dx][dy] = min(grid[dx][dy], new_t);
                        q.push_back(((dx, dy), new_t));
                    }
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if 0 == grid[i][j] {
                    bfs(&mut grid, (i, j), m, n);
                }
            }
        }
        // println!("{:?}", grid);
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == i32::MAX {
                    return -1;
                }
                ans = max(grid[i][j], ans);
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        let mut next_queue = Vec::new();
        let mut queue = Vec::new();

        let mut oranges = 0;
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                let orange = grid[r][c];
                match orange {
                    0 => continue,
                    1 => {
                        oranges += 1;
                    }
                    2 => {
                        oranges += 1;
                        next_queue.push((r, c));
                    }
                    _ => unreachable!(),
                };
            }
        }
        if oranges == 0 {
            return 0;
        }

        let mut time = -1;
        while !next_queue.is_empty() {
            assert!(queue.is_empty());
            std::mem::swap(&mut next_queue, &mut queue);
            while let Some((r, c)) = queue.pop() {
                if r > 0 && grid[r-1][c] == 1 {
                    next_queue.push((r-1, c));
                    grid[r-1][c] = 2;
                }
                if c > 0 && grid[r][c-1] == 1 {
                    next_queue.push((r, c-1));
                    grid[r][c-1] = 2;
                }
                if r < grid.len()-1 && grid[r+1][c] == 1 {
                    next_queue.push((r+1, c));
                    grid[r+1][c] = 2;
                }
                if c < grid[r].len()-1 && grid[r][c+1] == 1 {
                    next_queue.push((r, c+1));
                    grid[r][c+1] = 2;
                }

                oranges -= 1;
            }
            time += 1;
        }

        if oranges == 0 {
            time
        } else {
            -1
        }
    }
}

