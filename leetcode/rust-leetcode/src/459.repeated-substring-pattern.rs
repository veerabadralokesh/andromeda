impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let sb = s.into_bytes();
        // Linux
        let l = sb.len();
        for i in 1..=l/2 {
            if l % i == 0 {
                let mut flag = true;
                for j in i..l {
                    if sb[j] != sb[j-i] {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return flag;
                }
            }
        }
        false
    }
}

