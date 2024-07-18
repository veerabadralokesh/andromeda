impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let sb = s.into_bytes();
        let mut start = 0;
        let mut ans = Vec::new();
        for i in 1..=sb.len() {
            if i == sb.len() || sb[i] != sb[i-1]{
                if i - start > 2 {
                    ans.push([start as i32, i as i32-1].to_vec());
                }
                start = i;
            }
        }
        ans
    }
}

