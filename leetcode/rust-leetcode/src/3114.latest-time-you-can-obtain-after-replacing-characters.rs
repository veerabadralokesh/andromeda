impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut sb = s.into_bytes();
        if sb[0] == b'?' {
            if [b'0', b'1', b'?'].contains(&sb[1]) {
                sb[0] = b'1';
            } else {
                sb[0] = b'0';
            }
        }
        if sb[1] == b'?' {
            if [b'1', b'?'].contains(&sb[0]) {
                sb[1] = b'1';
            } else {
                sb[1] = b'9';
            }
        }
        if sb[3] == b'?' {sb[3] = b'5';}
        if sb[4] == b'?' {sb[4] = b'9';}
        sb.iter().map(|&b| b as char).collect()
    }
}

