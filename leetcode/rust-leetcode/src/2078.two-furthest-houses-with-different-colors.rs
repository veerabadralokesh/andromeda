impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, colors.len()-1);
        while colors[l] == colors[r] {
            l += 1;
        }
        while colors[r] == colors[0] {
            r -= 1;
        }
        r.max(colors.len()-1-l) as _
    }
}

