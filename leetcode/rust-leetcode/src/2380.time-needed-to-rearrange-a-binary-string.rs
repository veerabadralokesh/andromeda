impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut sb = s.into_bytes();
        let mut seconds = -1;
        let mut swapped = true;
        let mut i = 0;
        while swapped {
            seconds += 1;
            swapped = false;
            i = 1;
            while i < sb.len() {
                if sb[i] == b'1' && sb[i-1] == b'0' {
                    sb.swap(i, i-1);
                    i += 2;
                    swapped = true;
                } else {
                    i += 1;
                }
            }
        }
        seconds
    }
}

