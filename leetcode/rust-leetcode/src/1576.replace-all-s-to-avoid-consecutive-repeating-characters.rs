impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut sb = s.into_bytes();
        for i in 0..sb.len() {
            if sb[i] == b'?' {
               let mut c = b'a';
                while (i > 0 && sb[i-1] == c) || (i < sb.len()-1 && c == sb[i+1]) {
                    c += 1;
                }
                sb[i] = c;
            }
        }
        sb.iter().map(|&b| b as char).collect()
    }
}

