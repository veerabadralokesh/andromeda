impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for n in nums {
            match n {
                x if x > 9 && x < 100 => {ans += 1;},
                x if x > 999 && x < 10000 => {ans += 1;},
                x if x > 99999 => {ans += 1;},
                _ => {}
            }
        }
        ans
    }
}

