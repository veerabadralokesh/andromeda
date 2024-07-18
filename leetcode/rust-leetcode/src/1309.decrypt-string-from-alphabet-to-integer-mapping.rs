impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let sb = s.into_bytes();
        let l = sb.len();
        let mut ans = String::new();
        let mut i = 0;
        while i < l {
            if i < l-2 && b'#' == sb[i+2] {
                ans.push(((sb[i] - b'0') * 10 + sb[i+1] - b'0' + b'a' - 1) as char);
                i += 3;
            } else {
                ans.push((sb[i] - b'0' + b'a' - 1) as char);
                i += 1;
            }
        }
        ans
    }
}

