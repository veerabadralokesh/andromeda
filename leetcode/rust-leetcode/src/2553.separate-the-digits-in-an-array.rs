impl Solution {
    pub fn separate_digits(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        while let Some(mut num) = nums.pop() {
            while num > 0 {
                ans.push(num % 10);
                num /= 10;
            }
        }
        ans.reverse();
        ans
    }
}

