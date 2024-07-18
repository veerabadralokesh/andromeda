impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let (mut count, mut max_count) = (0, 0);
        let k = k as usize;
        let bb = blocks.into_bytes();
        for (i, &b) in bb.iter().enumerate() {
            if b == b'B' {
                count += 1;
            }
            if i >= k && bb[i-k] == b'B' {
                count -= 1;
            }
            max_count = count.max(max_count);
        }
        k as i32 - max_count
    }
}

