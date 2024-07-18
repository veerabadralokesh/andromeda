impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = "AEIOUaeiou";
        let mut svowels = s.chars().filter(|c| vowels.contains(*c)).collect::<Vec<char>>();
        // println!("{:?}", svowels);
        svowels.sort();
        let mut i = 0usize;
        let mut ans = String::new();
        for c in s.chars() {
            if vowels.contains(c) {
                ans.push(svowels[i]);
                i += 1;
            } else {
                ans.push(c);
            }
        }
        ans
    }
}

/*
*/

// LEARN

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<char> = s.chars().filter(|&c| "aeiouAEIOU".contains(c)).collect();
        vowels.sort_unstable();
        let mut vowels_iter = vowels.into_iter();
        s.chars()
            .map(|c| if "aeiouAEIOU".contains(c) { vowels_iter.next().unwrap() } else { c })
            .collect()
    }
}
