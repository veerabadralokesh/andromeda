impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut total = apple.iter().sum::<i32>();
        let mut ans = 0;
        capacity.sort_by_key(|&c| -c);
        let mut i = 0;
        while total > 0 {
            total -= capacity[i];
            i += 1;
            ans += 1;
        }
        ans
    }
}

