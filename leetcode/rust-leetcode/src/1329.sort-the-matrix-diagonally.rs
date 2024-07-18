impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = mat.to_vec();
        for d in 0..m {
            let mut diag = Vec::new();
            for j in 0..n {
                let i = d + j;
                if i < m {
                    diag.push(mat[i][j]);
                }
            }
            diag.sort();
            let mut k = 0;
            for j in (0..n) {
                let i = d + j;
                if i < m {
                    ans[i][j] = diag[k];
                    k += 1;
                }
            }
        }
        for d in 1..n {
            let mut diag = Vec::new();
            for i in 0..m {
                let j = d + i;
                if j < n {
                    diag.push(mat[i][j]);
                }
            }
            diag.sort();
            let mut k = 0;
            for i in 0..m {
                let j = d + i;
                if j < n {
                    ans[i][j] = diag[k];
                    k += 1;
                }
            }
        }
        ans
    }
}

/* */

// LEARN

use std::collections::HashMap;
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m=mat.len();
        let n=mat[0].len();
        let mut diagonal:HashMap<i32,Vec<i32>>=HashMap::new();

        for i in 0..m{
            for j in 0..n{
                let d=i as i32-j as i32;
                diagonal.entry(d).or_insert(Vec::new()).push(mat[i][j]);
            }
        }
        let mut sorted_mat = mat.clone();

        for (key, val) in &mut diagonal{
            val.sort_by(|x,y| y.cmp(x));
        }
        
        for i in 0..m{
            for j in 0..n{
                let d=i as i32-j as i32;
                sorted_mat[i][j]=diagonal.get_mut(&d).unwrap().pop().unwrap();
            }
        }
        sorted_mat
    }
}

