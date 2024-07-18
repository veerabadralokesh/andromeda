use std::cmp::min;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counts = [0; 26];
        for b in text.into_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        counts[(b'l' - b'a') as usize] /= 2;
        counts[(b'o' - b'a') as usize] /= 2;
        let mut count = counts[0];
        let indices = [b'b', b'a', b'l', b'o', b'n'];
        for idx in indices.into_iter() {
            if counts[(idx-b'a') as usize] == 0 || count == 0 {return 0;}
            count = min(counts[(idx-b'a') as usize], count);
        }
        count
    }
}

