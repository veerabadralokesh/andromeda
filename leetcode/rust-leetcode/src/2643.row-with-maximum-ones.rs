impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut max_index, mut max_count) = (0, 0);
        for (i, row) in mat.iter().enumerate() {
            let count = row.iter().sum::<i32>();
            if max_count < count {
                max_count = count;
                max_index = i as i32;
            }
        }
        vec![max_index, max_count]
    }
}

