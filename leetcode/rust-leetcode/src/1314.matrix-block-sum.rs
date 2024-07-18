impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = mat.to_vec();
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;
        for i in 0..m {
            for j in 0..n {
                ans[i as usize][j as usize] = 0;
                for x in ((i-k).max(0) as usize)..((i+k+1).min(m) as usize) {
                    for y in ((j-k).max(0) as usize)..((j+k+1).min(n) as usize) {
                        ans[i as usize][j as usize] += mat[x][y];
                    }
                }
            }
        }
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn prefixSum(prefix: &Vec<Vec<i32>>, i: usize, j: usize, n: usize, m: usize, k: usize) -> i32{
        let r1 = i.saturating_sub(k) + 1;
        let c1 = j.saturating_sub(k) + 1;
        let r2 = usize::min(i + k, n - 1) + 1;
        let c2 = usize::min(j + k, m - 1) + 1;
        prefix[r2][c2] - prefix[r2][c1 - 1] - prefix[r1 - 1][c2] + prefix[r1 - 1][c1 - 1]
    }
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (n, m) = (mat.len(), mat[0].len());
        let k = k as usize;
        let mut prefix = vec![vec![0; m + 1]; n + 1];
        let mut ans = vec![vec![0; m]; n];
        for i in 0..n{
            for j in 0..m{
                prefix[i + 1][j + 1] = mat[i][j] + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
            }
        }
        for i in 0..n{
            for j in 0..m{
                ans[i][j] = Self::prefixSum(&prefix, i, j, n, m, k);
            }
        }
        ans
    }
}
