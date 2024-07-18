use std::collections::VecDeque;
static directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        fn bfs(grid: &mut Vec<Vec<i32>>, start: (usize, usize), m: usize, n: usize) -> i32 {
            let mut q = VecDeque::new();
            q.push_back(start);
            let (mut area, mut di, mut dj) = (1, 0, 0);
            grid[start.0][start.1] = 2;
            while let Some((i, j)) = q.pop_front() {
                for dir in directions.iter() {
                    di = (i.saturating_add_signed(dir.0)).min(m);
                    dj = (j.saturating_add_signed(dir.1)).min(n);
                    if grid[di][dj] == 1 {
                        area += 1;
                        grid[di][dj] = 2;
                        q.push_back((di, dj));
                    }
                }
            }
            area
        }
        let (mut max_area, mut area) = (0, 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    area = bfs(&mut grid, (i, j), m-1, n-1);
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }
}

