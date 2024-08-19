impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let (mut ans, mut zeros, mut ones, mut l, mut r) = (0, 0, 0, 0, 0);
        for r in 0..s.len() {
            match &s[r..r+1] {
                "0" => zeros += 1,
                _ => ones += 1,
            }
            while l < r && zeros > k && ones > k {
                match &s[l..l+1] {
                    "0" => zeros -= 1,
                    _ => ones -= 1,
                }
                l +=1 ;
            }
            ans += (r - l + 1);
        }
        ans as _
    }
}

