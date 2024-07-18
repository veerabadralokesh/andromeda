use std::collections::{VecDeque,HashSet};
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let mut q = VecDeque::new();
        q.push_back((entrance.clone(), 0));
        let mut set = HashSet::new();
        set.insert(entrance);
        let xd = [-1, 0, 0, 1];
        let yd = [0, 1, -1, 0];
        let (m, n) = (maze.len(), maze[0].len());
        while let Some((pos, path)) = q.pop_front() {
            for i in 0..4 {
                let dx = xd[i];
                let dy = yd[i];
                if (pos.0 == 0 && dx < 0) || (pos.1 == 0 && dy < 0) {
                    continue;
                }
                let x = pos.0.saturating_add_signed(dx);
                let y = pos.1.saturating_add_signed(dy);
                if x >= m || y >= n || set.contains(&(x, y)) || maze[x][y] == '+' {
                    continue;
                }
                if (x == 0 || y == 0 || x == m-1 || y == n - 1) && maze[x][y] == '.' {
                    return path + 1;
                }
                set.insert((x, y));
                q.push_back(((x, y), path+1));
            }
        }
        -1
    }
}
