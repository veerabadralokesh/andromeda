impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut flags:Vec<i32> = s.into_bytes().into_iter().map(|c| if c == 97 || c == 101 || c == 105 || c==111||c==117 {1} else {0}).collect::<Vec<i32>>();
        let mut temp = flags[0..k].iter().sum();
        let mut ans = temp;
        for i in k..flags.len() {
            temp += flags[i] - flags[i-k];
            if temp > ans {
                ans = temp;
            }
        }
        ans
    }
}