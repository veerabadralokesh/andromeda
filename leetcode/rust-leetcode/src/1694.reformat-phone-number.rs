impl Solution {
    pub fn reformat_number(number: String) -> String {
        let digits = number.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
        let n = digits.len();
        let mut remaining = n;
        let mut ans = String::with_capacity(digits.len() * 2);
        while remaining > 0 {
            if remaining == 4 || remaining == 2 {
                for _ in 0..2 {
                    ans.push(digits[n-remaining]);
                    remaining -= 1;
                }
            } else {
                for _ in 0..3 {
                    ans.push(digits[n-remaining]);
                    remaining -= 1;
                }
            }
            if remaining > 0 {
                ans.push('-');
            }
        }
        ans
    }
}

