impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let bytes = strs.iter().map(|s| s.clone().into_bytes()).collect::<Vec<_>>();
        let (m, n) = (bytes.len(), bytes[0].len());
        let mut ans = 0;
        for j in 0..n {
            for i in 1..m {
                if bytes[i][j] < bytes[i-1][j] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}

