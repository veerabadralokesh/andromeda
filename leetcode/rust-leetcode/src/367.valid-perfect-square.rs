impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i: i32 = 1;
        let mut e: i32 = 1;
        while i - num / i > 0 {
            i = (i + num / i)/2;
            e = i - num / i;
        }
        i * i == num
    }
}