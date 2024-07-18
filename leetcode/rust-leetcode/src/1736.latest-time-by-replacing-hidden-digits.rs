impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut tb = time.into_bytes();
        if tb[0] == b'?' {
            if tb[1] == b'0' || tb[1] == b'1' || tb[1] == b'2' || tb[1] == b'3' || tb[1] == b'?' {
                tb[0] = b'2';
            } else {
                tb[0] = b'1';
            }
        }
        if tb[1] == b'?' {
            if tb[0] == b'0' || tb[0] == b'1' {
                tb[1] = b'9';
            } else {
                tb[1] = b'3';
            }
        }
        if tb[3] == b'?' {tb[3] = b'5';}
        if tb[4] == b'?' {tb[4] = b'9';}
        tb.iter().map(|&b| b as char).collect()
    }
}

