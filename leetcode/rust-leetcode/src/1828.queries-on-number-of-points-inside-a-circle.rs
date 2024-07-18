impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::with_capacity(queries.len());
        for c in queries.iter() {
            let mut num:i32 = 0;
            for p in points.iter() {
                if ((p[0]-c[0]).pow(2) + (p[1]-c[1]).pow(2) <= c[2].pow(2)) {
                    num += 1;
                }
            }
            ans.push(num);
        }
        ans
    }
}

impl Solution2 {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut totals: Vec<i32> = vec![];
        queries.iter().for_each(|q| {
            let circ: Circle = Circle::new(q);
            let mut inside_count = 0;
            points.iter().for_each(|p| {
                if circ.point_inside(p) {
                    inside_count += 1;
                }
            });

            totals.push(inside_count);
        });

        totals
    }
}

#[derive(Default)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl Circle {
    pub fn new(info: &[i32]) -> Circle {
        Circle {
            x: *info.get(0).unwrap(),
            y: *info.get(1).unwrap(),
            radius: *info.get(2).unwrap(),
        }
    }

        pub fn point_inside(&self, point: &[i32]) -> bool {
        let x = point.get(0).unwrap();
        let y = point.get(1).unwrap();

        (x - self.x).pow(2) + (y - self.y).pow(2) <= self.radius.pow(2)
    }

}