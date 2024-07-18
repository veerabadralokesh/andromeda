impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; heights.len()];

        let mut stack = Vec::new();
        let n = heights.len();
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] <= heights[i] {
                ans[stack.pop().unwrap()] += 1;
            }
            match stack.last() {
                Some(&idx) => {
                    ans[idx] += 1;
                },
                _ => {}
            };
            stack.push(i);
        }

        ans
    }
}

