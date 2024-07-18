impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let k = k as usize;
        let l = s.len();
        let sc = s.chars().collect::<Vec<_>>();
        (0..l).map(|i| sc[(i+k)%l]).collect()
    }
}

