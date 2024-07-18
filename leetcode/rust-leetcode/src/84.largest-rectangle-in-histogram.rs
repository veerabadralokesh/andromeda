impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut area = 0i32;
        let mut stack:Vec<usize> = Vec::with_capacity(heights.len());
        for i in 0..=heights.len() {
            while !stack.is_empty() && (i == heights.len() || heights[*stack.last().unwrap()] > heights[i]) {
                let h = heights[stack.pop().unwrap()];
                let w = if stack.is_empty() {i as i32} else {(i - *stack.last().unwrap() - 1) as i32};
                area = area.max(h * w);
            }
            stack.push(i);
        }
        area
    }
}
