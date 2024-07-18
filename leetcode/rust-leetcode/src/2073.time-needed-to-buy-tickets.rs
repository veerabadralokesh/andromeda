impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let kt = tickets[k];
        let mut time = kt;
        for i in 0..k {
            time += kt.min(tickets[i]);
        }
        for i in k+1..tickets.len() {
            time += tickets[i].min(kt-1);
        }
        time
    }
}

/* */

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        tickets.iter().enumerate().fold(0, |time, (i, ticket)| {
            time + (tickets[k] - i32::from(i > k)).min(*ticket)
        })
    }
}
