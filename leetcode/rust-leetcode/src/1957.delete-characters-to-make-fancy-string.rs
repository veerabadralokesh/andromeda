impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::with_capacity(s.len());
        let mut count = 0;
        let mut prev = 'A';
        for c in s.chars() {
            if c != prev {
                prev = c;
                count = 1;
                ans.push(c);
            } else if count < 2 {
                count += 1;
                ans.push(c);
            }
        }
        ans
    }
}

