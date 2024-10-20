impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }
        let midIndex = 2_i32.pow(n as u32 - 1);
        if k == midIndex {
            return '1';
        }
        if k < midIndex {
            return Self::find_kth_bit(n-1, k);
        }
        if Self::find_kth_bit(n-1, 2 * midIndex - k) == '0' {
            '1'
        } else {
            '0'
        }
    }
}

