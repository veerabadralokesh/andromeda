impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut counts = [0; 256];
        let (mut left, mut right, l, mut ans) = (0, 0, s.len(), 0);
        let sb = s.into_bytes();
        while right < l {
            let mut targeti = right;
            while right < l {
                let i = (sb[right] - 0u8) as usize;
                counts[i] += 1;
                right += 1;
                if counts[i] > 1 {
                    targeti = i;
                    break;
                }
                ans = ans.max(right - left);
            }
            while counts[targeti] > 1 {
                let i = (sb[left] - 0u8) as usize;
                counts[i] -= 1;
                left += 1;
                if counts[targeti] == 1 {
                    break;
                }
            }
        }
        ans as i32
    }
}
