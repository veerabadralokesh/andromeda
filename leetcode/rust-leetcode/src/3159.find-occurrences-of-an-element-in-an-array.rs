impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let indices = nums.iter().enumerate().filter(|(i, &n)| n == x).map(|(i, &n)| {
            i as i32
        }).collect::<Vec<_>>();
        queries.iter().map(|&q| {
            let q = q as usize;
            if q <= indices.len() {indices[q-1]}
            else {-1}
        }).collect::<Vec<i32>>()
    }
}

