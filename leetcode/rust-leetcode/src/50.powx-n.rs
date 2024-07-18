impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut sign = 1;
        let mut n:i64 = n as i64;
        let mut x = x;
        if n < 0 {
            x = 1.0/x;
            n = -n;
        }
        let mut ans:f64 = 1.0;
        while n > 0 {
            if n & 1 == 1 {
                ans *= x;
            }
            x *= x;
            n >>= 1;
        }
        ans
    }
}
