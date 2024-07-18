impl Solution {
    pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
        let (mut res, mut curr) = (0, 1);
        while n != 0 {
            if n % 2 == 1 {
                res = curr - res;
            }
            curr = 2 * curr + 1;
            n /= 2;
        }
        return res;
    }
}


/* */

// LEARN

impl Solution {
    fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let f_nxt = Self::minimum_one_bit_operations(n / 2);
        if n % 2 == f_nxt % 2 {
            2 * f_nxt
        } else {
            1 + 2 * f_nxt
        }
    }
}

/* */

impl Solution {
    fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        if Self::is_power_of_two(n) {
            return (((n as u64) << 1) - 1) as i32;
        }
        let f_nxt = Self::minimum_one_bit_operations(n / 2);
        if n % 2 == f_nxt % 2 {
            2 * f_nxt
        } else {
            1 + 2 * f_nxt
        }
    }
}
