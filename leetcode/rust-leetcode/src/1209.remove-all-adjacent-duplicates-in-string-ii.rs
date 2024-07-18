impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k = k as usize;
        if s.len() < k {
            return s;
        }
        let mut stack:Vec<(char,usize)> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if !stack.is_empty() {
                let last = *stack.last().unwrap();
                if last.0 == c && last.1 == k-1 {
                    for _ in 0..k-1 {
                        stack.pop();
                    }
                } else if last.0 == c {
                    stack.push((c, last.1 + 1));
                } else {
                    stack.push((c, 1));
                }
            } else {
                stack.push((c, 1));
            }
        }
        stack.iter().map(|e| e.0).collect()
    }
}

