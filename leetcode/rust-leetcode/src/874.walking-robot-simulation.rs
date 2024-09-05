use std::collections::HashSet;
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut ans = 0;

        let mut dir = 0;
        let mut x = 0;
        let mut y = 0;

        let obstacles: HashSet<(i32, i32)> = obstacles.into_iter().map(|o| (o[0], o[1])).collect();

        for c in commands {
            if c == -1 {
                dir = (dir + 1) % 4;
            } else if c == -2 {
                dir = (dir + 3) % 4;
            } else {
                for _ in 0..c {
                    match obstacles.get(&(x + dirs[dir].0, y + dirs[dir].1)) {
                        None => {
                            x += dirs[dir].0;
                            y += dirs[dir].1;
                        },
                        _ => {
                            break;
                        }
                    }
                }
                ans = ans.max(x * x + y * y);
            }
        }
        ans
    }
}

