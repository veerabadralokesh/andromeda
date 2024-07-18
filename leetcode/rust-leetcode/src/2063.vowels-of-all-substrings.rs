impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let vowels = "aeiou";
        let l = word.len();
        let mut ans = 0;
        for i in 0..l {
            if vowels.contains(&word[i..i+1]) {
                ans += (i + 1) * (l - i);
            }
        }
        ans as _
    }
}

