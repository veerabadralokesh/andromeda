impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut ans = String::with_capacity(s.len());
        let ones:usize = s.chars().filter(|c| *c == '1').count() as usize;
        for _ in 0..(ones - 1) {ans.push('1');}
        for _ in 0..(s.len() - ones) {ans.push('0');}
        ans.push('1');
        ans
    }
}
