use std::collections::HashSet;
impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut rows = vec![i32::MAX; m];
        let mut cols = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                rows[i] = rows[i].min(matrix[i][j]);
                cols[j] = cols[j].max(matrix[i][j]);
            }
        }
        let mut set:HashSet<_> = rows.into_iter().collect();
        let mut ans = Vec::new();
        for c in cols {
            if set.contains(&c) {
                ans.push(c);
            }
        }
        ans
    }
}
