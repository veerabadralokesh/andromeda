impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut xs: Vec<i32> = points.iter().map(|x| x[0]).collect::<Vec<i32>>();
        // println!("{:?}", xs);
        xs.sort();
        let mut m:i32 = 0;
        for i in 0..(xs.len()-1) {
            let d:i32 = xs[i+1] - xs[i];
            m = d.max(m);
        }
        m
    }
}
