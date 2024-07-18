impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let (m, n) = (mat.len(), mat[0].len());
        let mut weaker = Vec::with_capacity(m);
        for i in 0..m {
            let mut sum = 0;
            for j in 0..n {
                sum += mat[i][j];
            }
            weaker.push((sum, i));
        }
        weaker.sort();
        weaker[0..k].into_iter().map(|e| e.1 as i32).collect()
    }
}

