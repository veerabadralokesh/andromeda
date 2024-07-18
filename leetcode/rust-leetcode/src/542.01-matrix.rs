use std::collections::VecDeque;
use std::cmp::{min,max};
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    mat[i][j] = i32::MAX;
                }
            }
        }
        fn bfs(mat: &mut Vec<Vec<i32>>, pos: (usize, usize), m: usize, n: usize) {
            let mut q = VecDeque::new();
            q.push_back((pos, 0i32));
            let xd = [-1, 0, 0, 1];
            let yd = [0, -1, 1, 0];
            while let Some((p, t)) = q.pop_front() {
                let new_t = t + 1;
                for i in 0..4 {
                    let dx = p.0.saturating_add_signed(xd[i]);
                    let dy = p.1.saturating_add_signed(yd[i]);
                    if dx >= m || dy >= n || mat[dx][dy] == 0 {
                        continue;
                    }
                    if new_t < mat[dx][dy] {
                        mat[dx][dy] = min(mat[dx][dy], new_t);
                        q.push_back(((dx, dy), new_t));
                    }
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if 0 == mat[i][j] {
                    bfs(&mut mat, (i, j), m, n);
                }
            }
        }
        mat
    }
}

/* */

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let mut out = mat.clone();
    let mut visited = VecDeque::new();

    let m = out.len();
    let n = out[0].len();
    let max = m * n;

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                visited.push_back((i, j));
            } else {
                out[i][j] = max as i32;
            }
        }
    }

    let adjacencies = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((i, j)) = visited.pop_front() {
        for (mm, nm) in &adjacencies {
            let (m1, n1) = (i as i32 + mm, j as i32 + nm);

            if m1 >= 0 && m1 < m as i32 && n1 >= 0 && n1 < n as i32 {
                let (madj, nadj) = (m1 as usize, n1 as usize);

                if out[madj][nadj] > out[i][j] + 1 {
                    visited.push_back((madj, nadj));
                    out[madj][nadj] = out[i][j] + 1;
                }
            }
        }
    }

    out

    }
}
