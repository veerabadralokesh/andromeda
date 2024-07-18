impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut lower = [0; 26];
        let mut upper = [0; 26];
        for b in s.into_bytes() {
            if b > b'Z' {
                lower[(b - b'a') as usize] += 1;
            } else {
                upper[(b - b'A') as usize] += 1;
            }
        }
        let mut ans = 0;
        let mut add = 0;
        for l in lower.into_iter() {
            ans += 2 * (l/2);
            if l & 1 == 1 {
                add = 1;
            }
        }
        for u in upper.into_iter() {
            ans += 2 * (u/2);
            if u & 1 == 1 {
                add = 1;
            }
        }
        ans + add
    }
}


