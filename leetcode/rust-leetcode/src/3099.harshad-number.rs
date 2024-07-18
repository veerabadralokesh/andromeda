impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(mut x: i32) -> i32 {
        let mut sum = 0;
        let orig = x;
        while x > 0 {
            sum += x % 10;
            x /= 10;
        }
        if orig % sum == 0 {sum} else {-1}
    }
}
