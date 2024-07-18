impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut lc: i32 = 0;
        let mut rc: i32 = 0;
        let mut bc: i32 = 0;
        for c in s.chars() {
            if (c == 'R') {
                rc += 1;
            } else {
                lc += 1;
            }
            if (rc == lc) {
                bc += 1;
                rc = 0;
                lc = 0;
            }
        }
        return bc;
    }
}

impl Solution2 {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut bc: i32 = 0;
        for c in s.chars() {
            if (c == 'R') {
                sum += 1;
            } else {
                sum -= 1;
            }
            if (sum == 0) {
                bc += 1;
            }
        }
        return bc;
    }
}