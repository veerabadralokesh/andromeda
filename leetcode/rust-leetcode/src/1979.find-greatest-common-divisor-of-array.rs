impl Solution {
    fn calc_gcd(a: i32, b: i32) -> i32 {
        match b {
            0 => a,
            _ => Self::calc_gcd(b, a % b)
        }
    }
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (mut small, mut large) = (1000, 0);
        for n in nums {
            small = small.min(n);
            large = large.max(n);
        }
        Self::calc_gcd(small, large)
    }
}
