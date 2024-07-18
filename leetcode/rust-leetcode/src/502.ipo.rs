use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut cap_profit = capital.into_iter().zip(profits).collect::<Vec<(i32,i32)>>();
        cap_profit.sort();
        let mut heap = BinaryHeap::new();
        let mut i = 0;
        for p in 0..k {
            while i <  n && cap_profit[i].0 <= w {
                heap.push(cap_profit[i].1);
                i += 1;
            }
            if heap.is_empty() {
                break;
            }
            w += heap.pop().unwrap();
        }
        w
    }
}

