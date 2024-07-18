impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        let mut sc = Vec::with_capacity(s.len());
        let mut char_count = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    stack.push(char_count);
                },
                ')' => {
                    if let Some(k) = stack.pop() {
                        sc[k..].reverse();
                    }
                },
                _ => {
                    sc.push(c);
                    char_count += 1;
                }
            }
        }
        sc.iter().collect()
    }
}

