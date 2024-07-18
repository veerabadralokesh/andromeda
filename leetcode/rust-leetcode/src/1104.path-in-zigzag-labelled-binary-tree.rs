impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut level = 0;
        let mut x = label;
        while x > 0 {
            x >>= 1;
            level += 1;
        }
        let mut ans = Vec::with_capacity(32);
        x = label;
        level -= 1;
        while level > 0 {
            ans.push(x);
            x >>= 1;
            level -= 1;
            x ^= (2_i32.pow(level)-1);
        }
        ans.push(1);
        ans.reverse();
        ans
    }
}

