impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut x = 5;
        let mut ans = 0;
        while n / x > 0 && x < 10001 {
            ans += n / x;
            x = x * 5;
        }
        ans
    }
}

/* */

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            n = n / 5;
            ans += n;
        }
        ans
    }
}

