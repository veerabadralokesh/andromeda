impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let (mut max_diag, mut max_area) = (0, 0);
        for rect in dimensions {
            let diag = rect[0].pow(2) + rect[1].pow(2);
            if diag > max_diag {
                max_diag = diag;
                max_area = rect[0] * rect[1];
            } else if diag == max_diag && max_area < rect[0]*rect[1] {
                max_area = rect[0] * rect[1];
            }
        }
        max_area
    }
}

