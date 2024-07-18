use std::collections::HashMap;
impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % (group_size as usize) != 0 {
            return false;
        }
        let mut counter = HashMap::new();
        hand.sort();
        let mut keys = Vec::new();
        let mut prev_key = -1;
        for &h in hand.iter() {
            if prev_key != h {
                prev_key = h;
                keys.push(h);
            }
            *counter.entry(h).or_insert(0) += 1;
        }
        for &i in keys.iter() {
            while *counter.get(&i).unwrap() > 0 {
                for j in i..i+group_size {
                    let mut c = counter.entry(j).or_insert(0);
                    if *c == 0 {
                        return false;
                    }
                    *c -= 1;
                }
            }
        }
        true
    }
}

