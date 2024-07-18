impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut counts = [0; 3];
        let (mut l, mut left, mut ans) = (s.len(), 0, 0);
        let sb = s.into_bytes().into_iter().map(|b| (b - b'a') as usize).collect::<Vec<_>>();
        for (right, &b) in sb.iter().enumerate() {
            counts[b] += 1;
            while counts[0] > 0 && counts[1] > 0 && counts[2] > 0 {
                ans += (l - right);
                counts[sb[left]] -= 1;
                left += 1;
            }
        }
        ans as _
    }
}

