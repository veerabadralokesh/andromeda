impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.to_vec();
        expected.sort();
        expected.into_iter().zip(heights).filter(|(e, h)| e != h).count() as _
    }
}

