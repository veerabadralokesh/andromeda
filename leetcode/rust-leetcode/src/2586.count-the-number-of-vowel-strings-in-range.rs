impl Solution {
    pub fn is_vowel_string(word: String) -> i32 {
        let wb = word.into_bytes();
        let (f, l) = (wb[0] as char, wb[wb.len()-1] as char);
        if (f == 'a' || f == 'e' || f == 'i' || f == 'o' || f == 'u') && (l == 'a' || l == 'e' || l == 'i' || l == 'o' || l == 'u') {
            1
        } else {
            0
        }
    }
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        (left..=right).map(|i| Solution::is_vowel_string(words[i as usize].clone())).sum()
    }
}

