use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|&n| n.to_string()).collect::<Vec<_>>();
        nums.sort_by(|a, b| {
            let x = a.to_string() + b.as_str();
            let y = b.to_string() + a.as_str();
            if x > y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let mut ans = nums.into_iter().collect::<String>().trim_start_matches("0").to_string();
        if ans.is_empty() {
            String::from("0")
        } else {
            ans
        }
    }
}

