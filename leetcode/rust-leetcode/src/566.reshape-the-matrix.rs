impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        let (m, n) = (mat.len(), mat[0].len());
        if r * c != m * n {
            mat
        } else {
            let mut ans = vec![];
            let flatten_mat: Vec<i32> = mat
                        .iter()
                        .flat_map(|row| row.iter())
                        .cloned()
                        .collect();
            for i in 0..r {
                ans.push(flatten_mat[i*c..(i+1)*c].to_vec());
            }
            ans
        }
    }
}

/* */

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        let (m, n) = (mat.len(), mat[0].len());
        if r * c != m * n {
            mat
        } else {
            let mut ans = vec![vec![0;c];r];
            let (mut x, mut y) = (0, 0);
            for i in 0..r {
                for j in 0..c {
                    ans[i][j] = mat[x][y];
                    if y == n-1 {
                        y = 0;
                        x += 1;
                    } else {
                        y += 1;
                    }
                }
            }
            ans
        }
    }
}

