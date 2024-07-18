impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        let mut indices = vec![false; s.len()];
        for (i,c) in s.bytes().enumerate() {
            if c == b'(' {
                stack.push(i);
            } else if c == b')' {
                let prev = stack.last();
                if prev.is_some() {
                    indices[i] = true;
                    indices[*prev.unwrap()] = true;
                    stack.pop();
                }
            } else {
                indices[i] = true;
            }
        }
        s.chars().into_iter().enumerate().filter(|&(i,c)| indices[i]).map(|(i,c)| c).collect()
    }
}

/* */

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();
        let mut indices = vec![false; s.len()];
        for (i,c) in s.bytes().enumerate() {
            if c == b'(' {
                stack.push(i);
            } else if c == b')' {
                match (stack.pop()) {
                    Some(j) => {
                        indices[i] = true;
                        indices[j] = true;
                    }
                    None => {}
                }
            } else {
                indices[i] = true;
            }
        }
        s.chars().into_iter().enumerate().filter(|&(i,c)| indices[i]).map(|(i,c)| c).collect()
    }
}
