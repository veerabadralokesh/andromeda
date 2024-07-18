impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (a ^ b, (a & b) << 1);
        }
        return a
    }
}

