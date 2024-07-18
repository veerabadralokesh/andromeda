impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num > 9 {
            let mut x = 0;
            while num > 0 {
                x += num % 10;
                num /= 10;
            }
            num = x;
        }
        num
    }
}
