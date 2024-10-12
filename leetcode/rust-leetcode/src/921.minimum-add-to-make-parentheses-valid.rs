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

/* */

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut open, mut close) = (0, 0);
        for b in s.bytes() {
            if b == b'(' {
                open += 1;
            } else if open > 0 {
                    open -= 1;
            } else {
                close += 1;
            }
        }
        open + close
    }
}

