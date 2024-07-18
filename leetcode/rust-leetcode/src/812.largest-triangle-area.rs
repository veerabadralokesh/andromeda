impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let l = points.len();
        let dist = |p1: &Vec<i32>, p2: &Vec<i32>| -> f64 {
            f64::sqrt(((p1[0]-p2[0]).pow(2) + (p1[1]-p2[1]).pow(2)) as f64)
        };
        let mut max_area: f64 = 0.0;
        for i in 0..l-2 {
            let p1 = &points[i];
            for j in i+1..l-1 {
                let p2 = &points[j];
                let a = dist(&p1, &p2);
                for k in j+1..l {
                    let p3 = &points[k];
                    let b = dist(p2, p3);
                    let c = dist(p1, p3);
                    let s = (a + b + c)/2.0;
                    let area = f64::sqrt(s * (s-a) * (s-b) * (s-c));
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }
}

/* */

// LEARN

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut res: f64 = 0.0;
        let size: usize = points.len();
        for i in 0..(size - 2) {
            for j in (i + 1)..(size - 1) {
                for k in (j + 1)..size {
                    let value: i32 = points[i][0] * points[j][1] + points[j][0] * points[k][1] + points[k][0] * points[i][1]
                                    -points[i][0] * points[k][1] - points[j][0] * points[i][1] - points[k][0] * points[j][1];
                    let value: f64 = value.abs() as f64 / 2.0;
                    if value > res {res = value;}
                }
            }
        }
        return res;
    }
}


