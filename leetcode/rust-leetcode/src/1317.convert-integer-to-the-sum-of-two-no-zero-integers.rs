impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let is_no_zero = |x: i32| -> bool {
            let mut x = x;
            while x > 0 {
                if x % 10 == 0 {
                    return false;
                }
                x /= 10;
            }
            true
        };
        for i in 1..n/2+1 {
            if is_no_zero(i) && is_no_zero(n-i) {
                return vec![i, n-i];
            }
        }
        vec![1, n-1]
    }
}

