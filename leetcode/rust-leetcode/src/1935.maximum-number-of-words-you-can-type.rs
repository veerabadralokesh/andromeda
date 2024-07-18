impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken = [true; 26];
        for b in broken_letters.into_bytes() {
            broken[(b - b'a') as usize] = false;
        }
        text.split_whitespace().into_iter().filter(|w| {
            w.chars().into_iter().all(|c| broken[(c as u8 - b'a') as usize])
        }).count() as _
    }
}

