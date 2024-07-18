impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans:String = String::new();
        let mut braces = 0;
        for c in s.chars() {
            if c == '(' {
                braces += 1;
                if braces > 1 {
                    ans.push(c);
                }
            } else {
                braces -= 1;
                if braces > 0 {
                    ans.push(c);
                }
            }
        }
        ans
    }
}