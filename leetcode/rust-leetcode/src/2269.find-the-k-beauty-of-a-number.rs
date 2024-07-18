impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0;
        let n = num.to_string();
        let mut d = 0;
        for i in 0..n.len()-k+1 {
            d = *(&n[i..i+k].to_string().parse::<i32>().unwrap());
            if d > 0 && num % d == 0 {
                ans += 1;
            }
        }
        ans
    }
}

