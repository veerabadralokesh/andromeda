impl Solution {
    pub fn kth_factor(n: i32, mut k: i32) -> i32 {
        for i in 1..=n {
            if n % i == 0 {
                k -= 1;
                if k == 0 {
                    return i;
                }
            }
        }
        -1
    }
}

/* */

impl Solution {
    pub fn kth_factor(n: i32, mut k: i32) -> i32 {
        let mut i = 0;
        let mut x = 1;
        while x < (f32::sqrt(n as f32)) as i32 {
            if n % x == 0 {
                i += 1;
                if i == k {
                    return x;
                }
            }
            x += 1;
        }
        x = n / x;
        while x >= 1 {
            if n % x == 0 {
                i += 1;
                if i == k {
                    return n/x;
                }
            }
            x -= 1;
        }
        -1
    }
}
