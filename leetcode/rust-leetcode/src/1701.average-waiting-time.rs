impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let (mut time, mut waiting_time) = (0, 0);
        let customers = customers.iter().map(|c| vec![c[0] as i64, c[1] as i64]).collect::<Vec<_>>();
        for c in customers.iter() {
            (time, waiting_time) = (time.max(c[0]) + c[1], waiting_time + (time-c[0]).max(0) + c[1]);
            // println!("{:?}", waiting_time);
        }
        waiting_time as f64 / customers.len() as f64
    }
}

