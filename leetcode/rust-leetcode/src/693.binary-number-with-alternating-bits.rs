impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut bit = n & 1;
        while n > 0 {
            if bit != n & 1 {
                return false;
            }
            n >>= 1;
            bit = 1-bit;
        }
        true
    }
}

