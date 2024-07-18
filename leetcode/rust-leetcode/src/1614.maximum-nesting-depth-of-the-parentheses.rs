impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut md = 0i32;
        let mut d = 0i32;
        for b in s.bytes() {
            if b == b'(' {
                d += 1;
                md = md.max(d);
            } else if b == b')' {
                d -= 1;
            }
        }
        md
    }
}
