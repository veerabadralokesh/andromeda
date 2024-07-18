impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {return false;}
        let l = s1.len();
        let mut s1count = [0; 26];
        for b in s1.into_bytes() {
            s1count[(b - b'a') as usize] += 1;
        }
        let mut s2count = [0; 26];
        let sb = s2.into_bytes().iter().map(|&b| (b - b'a') as usize).collect::<Vec<_>>();
        for i in 0..l {
            s2count[sb[i]] += 1;
        }
        for i in l..sb.len() {
            if s1count == s2count {
                return true;
            }
            s2count[sb[i]] += 1;
            s2count[sb[i-l]] -= 1;
        }
        if s1count == s2count {
            return true;
        }
        false
    }
}

