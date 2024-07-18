impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = i32::MIN;
        for w in s.split_whitespace() {
            match w.parse::<i32>() {
                Ok(x) => {
                    if x <= prev {
                        return false;
                    }
                    prev = x;
                },
                Err(x) => {}
            }
        }
        true
    }
}

