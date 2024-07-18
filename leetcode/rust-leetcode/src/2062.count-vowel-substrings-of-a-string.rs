use std::collections::HashSet;
impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        if word.len() < 5 {
            return 0;
        }
        let wb = word.into_bytes();
        let mut ans = 0;
        let vowels = "aeiou".chars().map(|c| c as u8).collect::<HashSet<_>>();
        let mut set = HashSet::with_capacity(5);
        for i in 0..wb.len()-4 {
            for &b in wb[i..].iter().take_while(|b| vowels.contains(b)) {
                if set.len() == 5 {
                    ans += 1;
                } else {
                    set.insert(b);
                    if set.len() == 5 {
                        ans += 1;
                    }
                }
            }
            set.clear();
        }
        ans
    }
}

