impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let (mut ans, mut min_dist, mut dist) = (-1, i32::MAX, 0);
        for (i, p) in points.iter().enumerate() {
            if p[0] == x || p[1] == y {
                dist = (p[0]-x).abs() + (p[1]-y).abs();
                if dist < min_dist {
                    min_dist = dist;
                    ans = i as i32;
                }
            }
        }
        ans
    }
}

