use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map:HashMap<char, i32> = HashMap::with_capacity(s.len());
        for c in s.chars() {
            println!("{}", c);
            *map.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            let mut count = map.entry(c).or_insert(0);
            if *count == 0 {
                return c;
            }
            *count -= 1;
        }
        'e'
    }
}

impl Solution2 {
    pub fn find_the_difference(s: String, t: String) -> char {
     let mut char_count = [0; 26]; // Assuming only lowercase letters are used.
        // Count the frequency of each character in s.
        for ch in s.bytes() {
            char_count[(ch - b'a') as usize] += 1;
        }
        // Find the character in t that has one extra count.
        for ch in t.bytes() {
            let index = (ch - b'a') as usize;
            char_count[index] -= 1;
            if char_count[index] == -1 {
                return ch as char;
            }
        }
        // This should not happen if the problem's constraints are met,
        // but Rust requires a return value in all cases. We return the default null character.
        '\0'
    }
}
