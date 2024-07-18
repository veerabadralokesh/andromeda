impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut i, mut j) = (0, 0);
        let (m, n) = (mat.len(), mat[0].len());
        while i < m && j < n {
            if j < n-1 && mat[i][j+1] > mat[i][j] {
                j += 1;
            } else if i < m-1 && mat[i+1][j] > mat[i][j] {
                i += 1;
            } else if i > 0 && mat[i-1][j] > mat[i][j] {
                i -= 1;
            } else if j > 0 && mat[i][j-1] > mat[i][j] {
                j -= 1;
            } else {
                return vec![i as i32, j as i32];
            }
        }
        vec![0, 0]
    }
}

