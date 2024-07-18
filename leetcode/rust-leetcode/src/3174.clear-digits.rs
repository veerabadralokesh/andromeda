impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = String::with_capacity(s.len());
        for c in s.chars() {
            if c < 'a' && !stack.is_empty() {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack
    }
}

