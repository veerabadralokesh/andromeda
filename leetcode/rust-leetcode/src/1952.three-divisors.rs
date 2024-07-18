impl Solution {
    pub fn is_three(n: i32) -> bool {
        if n < 4 {return false;}
        // let sqrtn = f64::sqrt(n as f64) as i32;
        let mut ans = 0;
        // if sqrtn * sqrtn == n {ans += 1}
        for i in 1..=n {
            if n % i == 0 {
                ans += 1;
                if ans > 3 {
                    return false;
                }
            }
        }
        ans == 3
    }
}

