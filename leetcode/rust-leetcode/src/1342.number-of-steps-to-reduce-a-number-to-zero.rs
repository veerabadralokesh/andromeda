impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut ans = 0i32;
        while num > 1 {
            if num & 1 == 1 {
                ans += 2;
            } else {
                ans += 1;
            }
            num >>= 1;
        }
        if num > 0 {ans += 1;}
        ans
    }
}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        match num > 1 {
            true => (num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32,
            false => num,
        }
    }
}
