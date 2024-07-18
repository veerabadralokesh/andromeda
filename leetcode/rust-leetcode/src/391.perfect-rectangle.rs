use std::collections::HashSet;
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let rectangles = rectangles.iter().map(|r| [r[0] as i64, r[1] as i64, r[2] as i64, r[3] as i64]).collect::<Vec<_>>();
        let mut r = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
        let mut area_sum = 0;
        let area = |x: i64, y: i64, a: i64, b: i64| -> i64 {
            (a - x) * (b - y)
        };
        let (mut x, mut y, mut a, mut b) = (0, 0, 0, 0);
        let mut set = HashSet::new();
        let (mut p1, mut p2, mut p3, mut p4);
        for rect in rectangles.iter() {
            (x, y, a, b) = (rect[0], rect[1], rect[2], rect[3]);
            r.0 = r.0.min(x);
            r.1 = r.1.min(y);
            r.2 = r.2.max(a);
            r.3 = r.3.max(b);
            area_sum += area(x, y, a, b);
            p1 = (x, y);
            p2 = (x, b);
            p3 = (a, b);
            p4 = (a, y);
            if !set.insert(p1) {
                set.remove(&p1);
            }
            if !set.insert(p2) {
                set.remove(&p2);
            }
            if !set.insert(p3) {
                set.remove(&p3);
            }
            if !set.insert(p4) {
                set.remove(&p4);
            }
        }
        area(r.0, r.1, r.2, r.3) == area_sum &&
        set.len() == 4 &&
        set.contains(&(r.0, r.1)) &&
        set.contains(&(r.0, r.3)) &&
        set.contains(&(r.2, r.3)) &&
        set.contains(&(r.2, r.1))
    }
}

