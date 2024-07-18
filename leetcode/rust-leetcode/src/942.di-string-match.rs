impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut ans = Vec::with_capacity(s.len()+1);
        let mut minimum = 0;
        let mut maximum = s.len() as i32;
        for b in s.into_bytes() {
            if b == b'I' {
                ans.push(minimum);
                minimum += 1;
            } else {
                ans.push(maximum);
                maximum -= 1;
            }
        }
        ans.push(maximum);
        ans
    }
}

