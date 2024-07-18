impl Solution {
    pub fn bitwise_complement(num: i32) -> i32 {
        let mut num = num;
        let mut complement = 0;
        let mut i = 0;
        while num >= 0 {
            if num & 1 == 0 {
                complement |= (1 << i);
            }
            i += 1;
            num >>= 1;
            if num == 0 {
                break;
            }
        }
        complement
    }
}

