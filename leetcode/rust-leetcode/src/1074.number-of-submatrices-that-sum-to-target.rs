impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut psum = vec![vec![0; n+1]; m+1];
        for i in 0..m {
            for j in 0..n {
                psum[i+1][j+1] = matrix[i][j] + psum[i+1][j] + psum[i][j+1] - psum[i][j];
            }
        }
        let mut ans = 0;
        for x_bottom in 1..=m {
            for y_bottom in 1..=n {
                let psum_bottom = psum[x_bottom][y_bottom];
                for x_top in 1..=x_bottom {
                    let diff1 = psum[x_top-1][y_bottom];
                    for y_top in 1..=y_bottom {
                        if psum_bottom - diff1 - psum[x_bottom][y_top-1] + psum[x_top-1][y_top-1] == target {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}

