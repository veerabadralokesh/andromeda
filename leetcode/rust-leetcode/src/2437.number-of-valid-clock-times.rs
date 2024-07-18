impl Solution {
    pub fn count_time(time: String) -> i32 {
        let tb = time.as_bytes();
        let mut count = 1;
        
        if tb[3] == b'?' {count *= 6}
        if tb[4] == b'?' {count *= 10}

        if tb[0] == b'?' && tb[1] == b'?' {
            count *= 24;
        } else if tb[0] == b'?' {
            if tb[1] < b'4' {
                count *= 3;
            } else {
                count *= 2;
            }
        } else if tb[1] == b'?' {
            if tb[0] == b'2' {
                count *= 4;
            } else {
                count *= 10;
            }
        }
        count
    }
}

