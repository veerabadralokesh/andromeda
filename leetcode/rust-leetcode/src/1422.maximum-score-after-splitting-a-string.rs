impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (mut ones, mut zeros) = (0, 0);
        let s = s.into_bytes();
        let total = s.len();
        for &b in &s {
            if b == b'1' {
                ones += 1;
            }
        }
        let mut ans = 0;
        for i in 0..s.len()-1 {
            match s[i] {
                b'1' => ones -= 1,
                _ => zeros += 1,
            }
            ans = ans.max(ones+zeros)
        }
        ans
    }
}
