impl Solution {
    pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
        points.sort_by_key(|p| p[0]);
        let mut x1 = points[0][0];
        let mut rects = 1;
        for i in 1..points.len() {
            if x1 + w < points[i][0] {
                rects += 1;
                x1 = points[i][0];
            }
        }
        rects
    }
}

