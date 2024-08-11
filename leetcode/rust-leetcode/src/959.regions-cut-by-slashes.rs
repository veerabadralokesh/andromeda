impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let l = n * 3;
        let mut g = vec![vec![0; l]; l];

        for i in 0..n {
            for j in 0..n {
                match &grid[i][j..j+1] {
                    "/" => {
                        g[i*3][j * 3 + 2] = 1;
                        g[i*3 + 1][j * 3 + 1] = 1;
                        g[i*3 + 2][j * 3] = 1;
                    },
                    "\\" => {
                        g[i*3][j*3] = 1;
                        g[i*3 + 1][j*3 + 1] = 1;
                        g[i*3 + 2][j*3 + 2] = 1;
                    },
                    _ => {}
                }
            }
        }
        let mut ans = 0;

        for i in 0..l {
            for j in 0..l {
                if g[i][j] == 0 {
                    Solution::dfs(&mut g, i as i32, j as i32, l as i32);
                    ans += 1;
                }
            }
        }
        ans
    }

    fn dfs(g: &mut Vec<Vec<u8>>, i: i32, j: i32, l: i32) {
        if i < 0 || i == l || j < 0 || j == l {
            return;
        }
        if g[i as usize][j as usize] != 0 {
            return;
        }

        g[i as usize][j as usize] = 2;
        Solution::dfs(g, i+1, j, l);
        Solution::dfs(g, i-1, j, l);
        Solution::dfs(g, i, j+1, l);
        Solution::dfs(g, i, j-1, l);
    }
}

