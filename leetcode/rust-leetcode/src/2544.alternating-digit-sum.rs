impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut flag = 1;
        let mut x = 0;
        while n > 0 {
            x += flag * (n % 10);
            flag *= -1;
            n /= 10;
        }
        if flag > 0 {
            -x
        } else {
            x
        }
    }
}

