use std::collections::{VecDeque,HashSet};

static xd:[isize; 4] = [-1, 0, 0, 1];
static yd:[isize; 4] = [0, -1, 1, 0];

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n-1][n-1] == 1 {
            return 0;
        }
        let mut dist = vec![vec![0; n]; n];
        fn bfs_dist(dist: &mut Vec<Vec<i32>>, grid: &Vec<Vec<i32>>, n: usize) {
            let mut q = VecDeque::new();
            let mut set = HashSet::new();
            for i in 0..=n {
                for j in 0..=n {
                    if grid[i][j] == 1 {
                        q.push_back((i, j));
                        set.insert((i, j));
                    }
                }
            }
            let mut d = 0;
            while q.len() > 0 {
                let l = q.len();
                for _ in 0..l {
                    if let Some((x, y)) = q.pop_front() {
                        dist[x][y] = d;
                        for i in 0..4 {
                            let dx = x.saturating_add_signed(xd[i]).min(n);
                            let dy = y.saturating_add_signed(yd[i]).min(n);
                            if !set.contains(&(dx, dy)) {
                                q.push_back((dx, dy));
                                set.insert((dx, dy));
                            }
                        }
                    }
                }
                d += 1;
            }
            
        }
        bfs_dist(&mut dist, &grid, n-1);

        fn has_valid_path(dist: &Vec<Vec<i32>>, safeness: i32, n: usize) -> bool {
            if dist[0][0] < safeness {
                return false;
            }
            let mut q = VecDeque::new();
            let mut set = HashSet::new();
            q.push_back((0, 0));
            set.insert((0, 0));
            while let Some((x, y)) = q.pop_front() {
                if dist[x][y] < safeness {
                    continue;
                }
                if x == n && y == n {
                    return true;
                }
                for i in 0..4 {
                    let dx = x.saturating_add_signed(xd[i]).min(n);
                    let dy = y.saturating_add_signed(yd[i]).min(n);
                    if !set.contains(&(dx, dy)) {
                        q.push_back((dx, dy));
                        set.insert((dx, dy));
                    }
                }
            }
            false
        }

        let mut left = 0;
        let mut right = n * 2;
        while left < right {
            let mid = (left + right)/2;
            if has_valid_path(&dist, mid as i32, n-1) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        (left - 1) as i32
    }
}
