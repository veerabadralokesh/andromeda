use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        fn dfs(ring: Vec<char>, key: &Vec<char>, idx: usize, map: &mut HashMap<(Vec<char>, usize), i32>) -> i32 {
            if idx == key.len() {
                return 0;
            }
            let rc = ring.to_vec();
            if map.contains_key(&(rc.to_vec(), idx)) {
                return *map.get(&(rc, idx)).unwrap();
            }
            let mut rotations = i32::MAX;
            for i in 0..ring.len() {
                if ring[i] == key[idx] {
                    let rot = i.min(ring.len() - i) as i32;
                    let mut rotated_ring = ring.to_vec();
                    rotated_ring.rotate_left(i);
                    let rest = dfs(rotated_ring, key, idx + 1, map);
                    rotations = rotations.min(rot + rest);
                }
            }
            map.insert((ring, idx), rotations);
            rotations
        }
        let ring = ring.chars().collect::<Vec<_>>();
        let key = key.chars().collect::<Vec<_>>();
        let mut map:HashMap<(Vec<char>, usize), i32> = HashMap::new();
        dfs(ring, &key, 0, &mut map) + (key.len() as i32)
    }
}
