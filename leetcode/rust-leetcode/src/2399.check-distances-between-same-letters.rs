impl Solution {
    pub fn check_distances(s: String, mut distance: Vec<i32>) -> bool {
        let sb = s.into_bytes();
        for (i, &b) in sb.iter().enumerate() {
            match distance[(b-b'a') as usize] {
                -1 => continue,
                d @ _ => {
                    let idx = i + d as usize + 1;
                    if idx >= sb.len() || sb[idx] != b {return false};
                    distance[(b-b'a') as usize] = -1;
                }
            }
        }
        true
    }
}

