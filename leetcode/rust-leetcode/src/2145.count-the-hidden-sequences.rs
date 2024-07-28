impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix = vec![0; differences.len()+1];
        prefix[0] = differences[0] as i64;
        let (mut min, mut max) = (prefix[0], prefix[0]);
        for i in 1..prefix.len() {
            prefix[i] = prefix[i-1] + differences[i-1] as i64;
            min = min.min(prefix[i]);
            max = max.max(prefix[i]);
        }
        // println!("{min} {max} {:?}", prefix);
        0.max((upper-lower) as i64 - (max - min) + 1) as _
    }
}

