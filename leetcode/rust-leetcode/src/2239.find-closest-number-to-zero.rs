impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let mut dist = i32::MAX;
        for &n in nums.iter() {
            if n.abs() < dist {
                ans = n;
                dist = n.abs();
            } else if n.abs() == dist {
                ans = ans.max(n);
            }
        }
        ans
    }
}

