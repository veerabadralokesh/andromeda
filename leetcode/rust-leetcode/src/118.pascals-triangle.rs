impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pt = Vec::with_capacity(num_rows as usize);
        let mut row = vec![1];
        pt.push(row.to_vec());
        let mut new_row = vec![];
        for _ in 1..num_rows {
            new_row.push(1);
            for j in 1..row.len() {
                new_row.push(row[j] + row[j-1]);
            }
            new_row.push(1);
            pt.push(new_row.to_vec());
            row = new_row.to_vec();
            new_row.clear();
        }
        pt
    }
}

