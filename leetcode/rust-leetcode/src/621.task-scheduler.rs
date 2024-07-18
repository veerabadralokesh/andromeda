impl Solution {
    pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
        let mut interval = 1i32;
        let mut counts = [0i32; 26];
        let mut tasks = tasks.into_iter().map(|t| ((t as u8)-b'A') as usize).collect::<Vec<_>>();
        for t in &tasks {
            // counts.entry(*t).and_modify(|c| *c += 1).or_insert(1);
            counts[*t] += 1;
        }
        // println!("{:?}", counts);
        let max_count = *counts.iter().max().unwrap();
        interval = (max_count-1) * ((n+1) as i32) + (counts.iter().filter(|&c| *c == max_count).count() as i32);
        interval = interval.max(tasks.len() as i32);
        interval
    }
}

