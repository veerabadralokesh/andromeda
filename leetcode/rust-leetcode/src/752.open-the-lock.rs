use std::collections::{HashSet,VecDeque};
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let convert = |s: String| s.into_bytes().iter().map(|x| (x - (b'0' as u8))).collect::<Vec<_>>();
        let target = convert(target);
        let mut visited = HashSet::new();
        for d in deadends {
            visited.insert(convert(d));
        }
        let start = convert(String::from("0000"));
        if visited.contains(&start) {
            return -1;
        }
        if target == start {
            return 0;
        }
        let mut ans = 0;
        let mut q = VecDeque::new(); //pending combinations
        q.push_back(start);
        let mut next_slot = (0..10).collect::<Vec<u8>>();
        let mut prev_slot = (0..10).collect::<Vec<u8>>();
        next_slot.rotate_left(1);
        prev_slot.rotate_right(1);
        while !q.is_empty() {
            let n = q.len();
            for _ in 0..n {
                let mut current_combo = q.pop_front().unwrap();
                if current_combo == target {
                    return ans;
                }

                for wheel in 0..4 {
                    let mut next_combo = current_combo.to_vec();
                    next_combo[wheel] = next_slot[next_combo[wheel] as usize];
                    if !visited.contains(&next_combo) {
                        q.push_back(next_combo.to_vec());
                        visited.insert(next_combo);
                    }
                    let mut next_combo = current_combo.to_vec();
                    next_combo[wheel] = prev_slot[next_combo[wheel] as usize];
                    if !visited.contains(&next_combo) {
                        q.push_back(next_combo.to_vec());
                        visited.insert(next_combo);
                    }
                }
            }
            ans += 1;
        }
        -1
    }
}
