use std::collections::HashMap;
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut centers = HashMap::new();
        let points = points.iter().map(|p| vec![p[0] as i64, p[1] as i64]).collect::<Vec<_>>();
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let (x1, y1, x2, y2) = (
                    points[i][0], points[i][1],
                    points[j][0], points[j][1]
                );
                let (x3, y3) = ((x1 + x2), (y1 + y2));
                centers.entry((x3, y3)).or_insert(Vec::new()).push((x1, y1, x2, y2));
            }
        }
        // println!("{:?}", centers);
        let dist = |x1: i64, y1: i64, x2: i64, y2: i64| -> i64 {
            (x1-x2).pow(2) + (y1-y2).pow(2)
        };
        let mut min_area = i64::MAX;
        let mut area = 0;
        for (_, points) in centers.into_iter() {
            if points.len() < 2 {continue;}
            for i in 0..points.len()-1 {
                let (ax, ay, _, _) = points[i];
                for j in i+1..points.len() {
                    let (cx, cy, dx, dy) = points[j];
                    if (cx != ax && dx != ax) || (cy != ay && dy != ay) {
                        continue;
                    }
                    if (cy - ay) * (dy - ay) + (cx - ax) * (dx - ax) == 0 {
                        area = dist(ax, ay, cx, cy) * dist(ax, ay, dx, dy);
                        if area > 0 {
                            min_area = min_area.min(f64::sqrt(area as f64) as i64);
                        }
                    }
                }
            }
        }
        if min_area == i64::MAX {0} else {min_area as _}
    }
}

/* */

impl Solution {
   pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
    let mut map = std::collections::HashMap::new();
    for point in &points {
        map.entry(point[0]).or_insert(vec![]).push(point[1]);
    }
    let mut min_area = std::i32::MAX;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if points[i][0] == points[j][0] || points[i][1] == points[j][1] {
                continue;
            }
            if map[&points[i][0]].contains(&points[j][1]) && map[&points[j][0]].contains(&points[i][1]) {
                min_area = std::cmp::min(min_area, (points[i][0] - points[j][0]).abs() * (points[i][1] - points[j][1]).abs());
            }
        }
    }
    if min_area == std::i32::MAX {
        return 0;
    }
    min_area
}
}

