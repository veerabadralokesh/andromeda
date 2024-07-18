impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut count = 0;
        let mut kc = 0;
        for c in s.chars() {
            if c == ' ' {
                kc += 1;
            }
            if kc == k {
                break;
            }
            count += 1;
        }
        s.chars().take(count).collect()
    }
}