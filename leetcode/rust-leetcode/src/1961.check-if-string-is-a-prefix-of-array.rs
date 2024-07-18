impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let sb = s.into_bytes();
        let mut i = 0;
        for word in words {
            if word.len() > sb.len() - i {
                return false;
            }
            for b in word.into_bytes() {
                if sb[i] == b {
                    i += 1;
                } else {
                    return false;
                }
            }
            if i == sb.len() {
                return true;
            }
        }
        false
    }
}

