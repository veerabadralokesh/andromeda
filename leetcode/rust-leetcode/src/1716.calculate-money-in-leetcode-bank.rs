impl Solution {
    pub fn total_money(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        while n > 7 {
            ans += 7 * (4 + i);
            n -= 7;
            i += 1;
        }
        ans += (n * (n+1))/2 + n * i;
        ans
    }
}
