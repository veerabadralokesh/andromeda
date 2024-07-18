impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut working = [0; 1002];
        for (s, e) in start_time.into_iter().zip(end_time) {
            working[s as usize] += 1;
            working[e as usize + 1] -= 1;
        }
        let query = query_time as usize;
        for i in 1..=query {
            working[i] += working[i-1];
        }
        working[query]
    }
}

