impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let (mut i, mut j, n) = (0usize, s.len()-1, s.len());
        let sb = s.into_bytes();
        while i < j {
            let c = sb[i];
            if sb[j] != c {
                break;
            }
            while sb[i] == c && i < j {
                i += 1;
            }
            while sb[j] == c && j >= i {
                j -= 1;
            }
        }
        (j - i + 1) as i32
    }
}
