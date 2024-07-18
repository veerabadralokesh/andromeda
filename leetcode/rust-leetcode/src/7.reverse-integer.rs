impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans = 0;
        let mut sign = 1;
        if x < 0 {
            x = -x;
            sign = -1;
        }
        while x > 0 {
            if ans > i32::MAX/10 {
                return 0;
            }
            ans = (ans * 10) + (x % 10);
            x = x/10;
        }

        ans * sign
    }
}
