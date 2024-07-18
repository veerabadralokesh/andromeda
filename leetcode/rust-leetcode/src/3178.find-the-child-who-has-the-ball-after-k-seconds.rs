impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let k = k % (2 * (n - 1));
        if k > (n-1) {
            (n << 1) - 2 - k
        } else {
            k
        }
    }
}


