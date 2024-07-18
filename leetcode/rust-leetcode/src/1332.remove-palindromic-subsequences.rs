impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let mut ans = 1;
        let sb = s.into_bytes();
        for i in 0..=sb.len()/2 {
            if sb[i] != sb[sb.len()-i-1] {
                ans = 2;
                break;
            }
        }
        ans
    }
}

