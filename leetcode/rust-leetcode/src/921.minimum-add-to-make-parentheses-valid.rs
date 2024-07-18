impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::new();
        for b in s.bytes() {
            if b == b'(' {
                stack.push(b);
            } else if b == b')' {
                if stack.len() > 0  && *stack.last().unwrap() == b'(' {
                    stack.pop();
                } else {
                    stack.push(b);
                }
            }
        }
        stack.len() as i32
    }
}
