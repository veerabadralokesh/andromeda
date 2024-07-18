impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for idx in indices {
            rows[idx[0] as usize] += 1;
            cols[idx[1] as usize] += 1;
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if (rows[i] + cols[j]) & 1 == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
