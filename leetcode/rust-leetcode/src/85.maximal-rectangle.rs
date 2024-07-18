impl Solution {
    pub fn largest_rectangle_area(heights: &Vec<i32>, area: &mut i32) {
        let mut stack:Vec<usize> = Vec::with_capacity(heights.len());
        for i in 0..=heights.len() {
            while !stack.is_empty() && (i == heights.len() || heights[*stack.last().unwrap()] > heights[i]) {
                let h = heights[stack.pop().unwrap()];
                let w = if stack.is_empty() {i as i32} else {(i - *stack.last().unwrap() - 1) as i32};
                *area = (*area).max(h * w);
            }
            stack.push(i);
        }
    }
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut matrix = matrix.iter().map(|r| r.iter().map(|&rc| ((rc as u8) - b'0') as i32).collect::<Vec<_>>()).collect::<Vec<_>>();
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ans = 0i32;
        Self::largest_rectangle_area(&matrix[0], &mut ans);
        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] > 0 {
                    matrix[i][j] = matrix[i-1][j]+1;
                }
            }
            Self::largest_rectangle_area(&matrix[i], &mut ans);
        }
        ans
    }
}
