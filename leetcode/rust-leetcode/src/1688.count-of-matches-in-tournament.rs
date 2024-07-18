impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut t:i32 = 0;
        while n > 1 {
            if n & 1 == 0 {
                n >>= 1;
                t += n;
            } else {
                n = (n-1)/2 + 1;
                t += n-1;
            }
        }
        t
    }
}

impl Solution2 {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1   
    }
}

impl Solution3 {
    pub const fn number_of_matches(n: i32) -> i32 {
        let halved = n >> 1;
        if n <= 1 {
            0
        } else {
            halved + Self::number_of_matches(halved + ((n & 1 == 1) as i32))
        }
    }
}
