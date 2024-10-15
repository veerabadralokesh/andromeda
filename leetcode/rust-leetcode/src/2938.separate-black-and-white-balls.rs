impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let (mut ones, mut ans) = (0,0);
        for b in s.bytes() {
            match b {
                b'1' => ones+=1,
                _ => ans += ones,
            }
        }
        ans
    }
}

