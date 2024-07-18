impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = f64::sqrt(area as f64) as i32;
        let mut l = area / w;
        while w * l != area {
            w -= 1;
            l = area / w;
        }
        vec![l, w]
    }
}

