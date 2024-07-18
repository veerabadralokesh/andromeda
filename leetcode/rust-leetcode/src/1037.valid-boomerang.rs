impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let (ax, ay) = (points[0][0], points[0][1]);
        let (bx, by) = (points[1][0], points[1][1]);
        let (cx, cy) = (points[2][0], points[2][1]);
        ((ax - bx) * (cy - by) - (cx - bx) * (ay - by)).abs() > 0
    }
}

