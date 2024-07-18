impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let word_to_num = |w: String| -> i32 {
            w.chars().map(|c| (c as u8 - b'a' + b'0') as char).collect::<String>().parse::<i32>().unwrap()
        };
        word_to_num(first_word) + word_to_num(second_word) == word_to_num(target_word)
    }
}

