use std::collections::{VecDeque,HashSet};
impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len() as i32, mat[0].len() as i32);
        let mut mask = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i as usize][j as usize] == 1 {
                    mask |= (1 << (i * n + j));
                }
            }
        }
        if mask == 0 {
            return 0;
        }

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(mask);
        visited.insert(mask);
        let mut steps = 0;
        let (mut x, mut y) = (0, 0);
        while !q.is_empty() {
            steps += 1;
            for _ in 0..q.len() {
                if let Some(state) = q.pop_front() {
                    for i in 0..m {
                        for j in 0..n {
                            let mut next = state ^ (1 << (i * n + j));
                            for &(dx, dy) in directions.iter() {
                                x = i + dx;
                                y = j + dy;
                                if x < 0 || x == m || y < 0 || y == n {
                                    continue;
                                }
                                next ^= (1 << (x * n + y));
                            }
                            if next == 0 {
                                return steps;
                            }
                            if visited.contains(&next) {
                                continue;
                            }
                            visited.insert(next);
                            q.push_back(next);
                        }
                    }
                }
            }
        }

        -1
    }
}

