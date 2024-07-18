impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if !stack.is_empty() && *stack.last().unwrap() == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.iter().collect()
    }
}

