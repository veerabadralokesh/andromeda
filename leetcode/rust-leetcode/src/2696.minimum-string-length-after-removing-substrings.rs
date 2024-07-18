impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        for b in s.chars() {
            if !stack.is_empty() && (
                b == 'B' && *stack.last().unwrap() == 'A' || 
                b == 'D' && *stack.last().unwrap() == 'C'
            ) {
                stack.pop();
                continue;
            }
            stack.push(b);
        }
        stack.len() as _
    }
}

