impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut sb = s.into_bytes().iter().map(|&b| (b -  b'0') as u8).collect::<Vec<_>>();
        let mut count = 0;
        for i in 1..sb.len() {
            if sb[i] == sb[i-1] {
                sb[i] = 1 - sb[i-1];
                count += 1;
            }
        }
        count.min(sb.len() as i32 - count)
    }
}

