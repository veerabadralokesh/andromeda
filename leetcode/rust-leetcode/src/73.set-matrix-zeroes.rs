impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_col = [false, false];
        for i in 0..m {if matrix[i][0]==0{row_col[0] = true;break;}}
        for i in 0..n {if matrix[0][i]==0{row_col[1] = true;break;}}
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }
        if row_col[0] {
            for i in 0..m {matrix[i][0] = 0;}
        }
        if row_col[1] {
            for i in 0..n {matrix[0][i] = 0;}
        }
    }
}

/*
*/

impl Solution2 {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut rows:Vec<bool> = Vec::with_capacity(m);
        let mut cols:Vec<bool> = Vec::with_capacity(n);
        for _ in 0..m {rows.push(false);}
        for _ in 0..n {cols.push(false);}
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }
        for i in 0..m {
            if rows[i] {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 0..n {
            if cols[j] {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

/* */

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row = false;
        let mut col = false;
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            if matrix[i][0] == 0 {
                col = true;
                break;
            }
        }
        for j in 0..n {
            if matrix[0][j] == 0 {
                row = true;
                break;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
        if col {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

