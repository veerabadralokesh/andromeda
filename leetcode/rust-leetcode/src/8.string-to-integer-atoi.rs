impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign = 1i64;
        let mut allow_sign_change = true;
        let mut leading_zeros = true;

        let LOWER_BOUND:i64 = - (1 << 31);
        let UPPER_BOUND:i64 = (1 << 31) - 1;

        let is_number = |b: u8| (b-b'0' >= 0) && (b-b'0' <= 10);
        let is_alphabet = |b: u8| ((b-b'a' >= 0) && (b-b'a' <= 26)) || ((b-b'A' >= 0) && (b-b'A' <= 26)) || (b-b'.' == 0);

        let mut ns = String::new();
        
        for b in s.bytes() {
            if (b == b' ') {
                if allow_sign_change && leading_zeros {
                    continue;
                } else {
                    break;
                }
            } else if is_number(b) {
                allow_sign_change = false;
                if leading_zeros && b == b'0' {
                    continue;
                } else {
                    leading_zeros = false;
                    ns.push((b as char));
                }
            } else if b == b'+' {
                if allow_sign_change {
                    allow_sign_change = false;
                } else {
                    break;
                }
            } else if b == b'-' {
                if allow_sign_change {
                    allow_sign_change = false;
                    sign = -1;
                } else {
                    break;
                }
            } else if is_alphabet(b) {
                break;
            }
        }
        if ns.len() == 0 {
            return 0i32;
        }
        if ns.len() > 10 {
            if sign == 1 {
                return UPPER_BOUND as i32;
            } else {
                return LOWER_BOUND as i32;
            }
        }
        let ans = ns.parse::<i64>().unwrap() * sign;
        if ans >= UPPER_BOUND {
            return UPPER_BOUND as i32;
        }
        if ans <= LOWER_BOUND {
            return LOWER_BOUND as i32;
        }
        (ans as i32)
    }
}
