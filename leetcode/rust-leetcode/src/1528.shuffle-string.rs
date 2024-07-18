impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut restored_string = vec!['a'; s.len()];
        for (i, c) in s.chars().enumerate() {
            restored_string[indices[i] as usize] = c;
        }
        restored_string.iter().collect()
    }
}

