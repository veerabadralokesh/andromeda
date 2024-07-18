impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut stack = Vec::new();
        let mut counts = [0; 26];
        let s = s.into_bytes();
        for b in &s {
            counts[(b - b'a') as usize] += 1;
        }
        let mut added = [false; 26];
        let n = s.len();
        for i in 0..n {
            let idx = (s[i] - b'a') as usize;
            while !stack.is_empty() && (s[*stack.last().unwrap()] >= s[i] && counts[(s[*stack.last().unwrap()] - b'a') as usize] > 1) && !added[idx] {
                let idx1 = (s[stack.pop().unwrap()] - b'a') as usize;
                added[idx1] = false;
                counts[idx1] -= 1;
            }
            if !added[idx] {
                stack.push(i);
                added[idx] = true;
            } else {
                counts[idx] -= 1;
            }
        }
        stack.iter().map(|&i| (s[i]) as char).collect()
    }
}
