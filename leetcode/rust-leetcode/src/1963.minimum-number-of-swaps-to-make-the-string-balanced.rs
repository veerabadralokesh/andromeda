impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut swap_pose = None;
        let mut sb = s.into_bytes();
        let mut stack = vec![];
        let mut count = 0;
        for i in 0..sb.len() {
            if sb[i] == b'[' {
                stack.push(i);
            } else if !stack.is_empty() {
                stack.pop();
            } else {
                if swap_pose.is_none() {
                    swap_pose = Some(i);
                    count += 1;
                } else {
                    let temp = sb[swap_pose.unwrap()];
                    sb[swap_pose.unwrap()] = sb[i];
                    sb[i] = temp;
                    swap_pose = None;
                }
            }
        }
        count
    }
}

