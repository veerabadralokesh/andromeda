impl Solution {
    pub fn two_egg_drop(mut n: i32) -> i32 {
        let mut ans = 1i32;
        let mut decrement = 1i32;
        while n > decrement {
            n -= decrement;
            ans += 1;
            decrement += 1;
        }
        ans
    }
}

/* */

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let mut ans = 1i32;
        while ans * (ans + 1) / 2 < n {
            ans += 1;
        }
        ans
    }
}

