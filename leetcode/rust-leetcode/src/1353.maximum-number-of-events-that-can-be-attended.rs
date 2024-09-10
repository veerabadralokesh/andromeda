use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort();
        let mut count = 0;
        let mut day = 0;
        
        let mut i = 0;
        let mut minHeap = BinaryHeap::new();
        while i < events.len() || !minHeap.is_empty() {
            if minHeap.is_empty() {
                day = events[i][0];
            }
            while i < events.len() && events[i][0] == day {
                minHeap.push(Reverse(events[i][1]));
                i += 1;
            }
            minHeap.pop();
            count += 1;
            day += 1;

            while !minHeap.is_empty() && minHeap.peek().unwrap().0 < day {
                minHeap.pop();
            }
        }
        count
    }
}

