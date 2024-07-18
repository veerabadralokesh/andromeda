impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[m-1].len();
        let mut transpose:Vec<Vec<i32>> = Vec::with_capacity(n);
        for i in 0..n {
            transpose.push(Vec::with_capacity(m));
            for j in 0..m {
                transpose[i].push(matrix[j][i]);
            }
        }
        transpose
    }
}