impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ans:String = String::new();
        for c in s.chars() {
            if c == '*' {
                if s.len() > 0 {
                    ans.pop();
                }
            } else {
                ans.push(c);
            }
        }
        ans
    }
}
