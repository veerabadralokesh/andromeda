impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut start = [None; 26];
        let mut ans = -1;
        for (i, b) in s.into_bytes().into_iter().enumerate() {
            let idx = (b - b'a') as usize;
            if start[idx].is_some() {
                ans = ans.max((i - start[idx].unwrap()) as i32 - 1);
            } else {
                start[idx] = Some(i);
            }
        }
        ans
    }
}

