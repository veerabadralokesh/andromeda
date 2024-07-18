impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::new();
        fn is_self_dividing(n: i32) -> bool {
            let mut x = n;
            let mut reminder = 0;
            while x > 0 {
                reminder = x % 10;
                if reminder == 0 || n % reminder != 0 {
                    return false;
                }
                x = x / 10;
            }
            true
        }
        for x in left..=right {
            if x % 10 == 0 {
                continue;
            }
            if is_self_dividing(x) {
                ans.push(x);
            }
        }
        ans
    }
}
