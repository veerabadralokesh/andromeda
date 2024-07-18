impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut counts = [0; 26];
        let sb = s.into_bytes().iter().map(|&b| (b - b'a') as usize).collect::<Vec<_>>();
        for &b in sb.iter() {
            counts[b] += 1;
        }
        let mut set = std::collections::HashSet::new();
        let mut ans = Vec::new();
        let mut partition = 0;
        for (i, &b) in sb.iter().enumerate() {
            partition += 1;
            counts[b] -= 1;
            if counts[b] > 0 {
                set.insert(b);
            } else if counts[b] == 0 && set.contains(&b) {
                set.remove(&b);
            }
            if set.len() == 0 || i == sb.len()-1 {
                ans.push(partition);
                partition = 0;
            }
        }
        ans
    }
}

