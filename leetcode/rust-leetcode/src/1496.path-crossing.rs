use std::collections::HashSet;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut set = HashSet::new();
        let mut pos = (0, 0);
        set.insert(pos);
        for b in path.into_bytes() {
            match b {
                b'N' => {
                    pos.1 += 1;
                },
                b'S' => {
                    pos.1 -= 1;
                },
                b'E' => {
                    pos.0 += 1;
                },
                b'W' => {
                    pos.0 -= 1;
                }
                _ => {
                    unreachable!();
                }
            }
            if set.contains(&pos) {
                return true;
            }
            set.insert(pos);
        }
        false
    }
}

