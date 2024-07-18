impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                rows[i] += mat[i][j];
                cols[j] += mat[i][j];
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if 1 == rows[i] && cols[j] == 1 && 1 == mat[i][j] {
                    ans += 1;
                }
            }
        }
        ans
    }
}

