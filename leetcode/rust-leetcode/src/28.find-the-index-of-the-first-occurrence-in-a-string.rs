impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(needle.as_str()) {
            Some(i) => i as i32,
            _ => -1
        }
    }
}
