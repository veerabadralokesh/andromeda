impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open = 0;
        let mut close = 0;
        let sb = s.into_bytes();
        for i in 0..sb.len() {
            let b = sb[i];
            let lb = sb[sb.len()-i-1];
            if b == b'(' || b == b'*' {
                open += 1;
            } else {
                open -= 1;
            }
            if lb == b')' || lb == b'*' {
                close += 1;
            } else {
                close -= 1;
            }
            if open < 0 || close < 0 {
                return false;
            }
        }
        true
    }
}

/* */

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open = 0;
        let mut close = 0;
        for b in s.into_bytes() {
            if b == b'(' {
                open += 1;
                close += 1;
            } else if b == b')' {
                open -= 1;
                if close > 0 {
                    close -= 1;
                }
            } else {
                open += 1;
                if close > 0 {
                    close -= 1;
                }
            }
            if open < 0 {
                return false;
            }
        }
        close == 0
    }
}
