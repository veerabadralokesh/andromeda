impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut counts = [[0; 3];10];
        let rb = rings.as_bytes();
        for i in (0..rb.len()).step_by(2) {
            let idx = match rb[i] {
                b'R' => 0,
                b'G' => 1,
                b'B' => 2,
                _ => unreachable!(),
            };
            counts[(rb[i+1]-b'0') as usize][idx] = 1;
        }
        let mut count = 0;
        for c in counts.iter() {
            if c[0] == 1 && c[1] == 1 && c[2] == 1 {
                count += 1;
            }
        }
        count
    }
}

