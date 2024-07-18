impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let (mut max_task, mut task_end, mut max_id) = (0, 0, 0);
        logs.iter().for_each(|log| {
            if let &[id, time] = log.as_slice() {
                let duration = time - task_end;
                if duration > max_task || duration == max_task && id < max_id {
                    max_task = duration;
                    max_id = id;
                }
                task_end = time;
            }
        });
        max_id
    }
}

