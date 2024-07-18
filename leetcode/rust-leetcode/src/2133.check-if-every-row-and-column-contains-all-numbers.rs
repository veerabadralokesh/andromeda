impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let mut row = vec![false; n+1];
        let mut col = vec![false; n+1];
        for i in 0..n {
            for j in 0..n {
                let ridx = matrix[i][j] as usize;
                let cidx = matrix[j][i] as usize;
                if row[ridx] || col[cidx] {
                    return false;
                }
                row[ridx] = true;
                col[cidx] = true;
            }
            for k in 1..=n {
                row[k] = false;
                col[k] = false;
            }
        }
        true
    }
}

