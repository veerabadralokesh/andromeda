impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut sc = s.chars().collect::<Vec<char>>();
        sc[..s.len()/2].iter().filter(|c| "aeiouAEIOU".contains(**c)).count() == sc[s.len()/2..].iter().filter(|c| "aeiouAEIOU".contains(**c)).count()
    }
}

/* */

// LEARN

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.as_bytes();
        let vowels = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
        let is_vowels = |b: &u8| vowels.iter().any(|v| v == b);
        s.iter().take(s.len()/2).filter(|b| is_vowels(b)).count() == s.iter().skip(s.len()/2).filter(|b| is_vowels(b)).count()
    }
}

