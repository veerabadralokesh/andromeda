use std::collections::VecDeque;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = vec![false;rooms.len()];
        queue.push_back(0);
        while let Some(r_index) = queue.pop_front() {
            if !visited[r_index] {
                visited[r_index] = true;
                for key in &rooms[r_index] {
                    let k = *key as usize;
                    if !visited[k] {
                        queue.push_back(k);
                    }
                }
            }
        }
        visited.into_iter().all(|b| b)
    }
}
