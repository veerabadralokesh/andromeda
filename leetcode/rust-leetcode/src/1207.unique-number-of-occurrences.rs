use std::collections::HashMap;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map:HashMap<i32,i32> = HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut counts = map.values().cloned().collect::<Vec<i32>>();
        counts.sort();
        for i in 1..counts.len() {
            if counts[i] == counts[i-1] {
                return false;
            }
        }
        true
    }
}

impl Solution2 {
    pub fn unique_occurrences(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut num_occurances = (1..arr.len()).fold(vec![1], |mut acc, i| {
            match arr[i - 1] == arr[i] {
                true => *acc.last_mut().unwrap() += 1,
                false => acc.push(1),
            }
            acc
        });
        num_occurances.sort();
        for window in num_occurances.windows(2) {
            if window[0].eq(&window[1]) {
                return false;
            }
        }
        true
    }
}
