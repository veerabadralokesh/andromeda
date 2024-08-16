impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for b in s.into_bytes() {
            match b {
                b'(' | b'[' | b'{' => stack.push(b),
                b')' => {
                    if stack.is_empty() || stack.pop().unwrap() != b'(' {
                        return false;
                    }
                },
                b']' => {
                    if stack.is_empty() || stack.pop().unwrap() != b'[' {
                        return false;
                    }
                },
                b'}' => {
                    if stack.is_empty() || stack.pop().unwrap() != b'{' {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}

