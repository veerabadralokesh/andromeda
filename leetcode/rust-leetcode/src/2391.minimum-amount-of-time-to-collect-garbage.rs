use std::collections::HashMap;
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, mut travel: Vec<i32>) -> i32 {
        for i in 1..travel.len() {
            travel[i] += travel[i-1];
        }
        // println!("{:?}", travel);
        let mut track:HashMap<u8, i32> = HashMap::with_capacity(3);
        let mut counts:HashMap<u8, i32> = HashMap::with_capacity(3);
        let mut time = 0i32;
        let mut i = 0i32;
        for house in garbage {
            for g in house.bytes() {
                *counts.entry(g).or_insert(0i32) += 1;
                track.insert(g, i);
            }
            i += 1;
        }
        // println!("{:?}", track);
        // println!("{:?}", counts);
        for truck in [b'G', b'P', b'M'] {
            time += counts.get(&truck).unwrap_or(&0i32);
            i = *(track.get(&truck).unwrap_or(&0i32));
            if i > 0 {
                time += travel[(i-1) as usize];
            }
        }
        time
    }
}

/* */

// LEARN

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, mut travel: Vec<i32>) -> i32 {
        let mut last = [0; 10];
        let mut res = 0;
        for (i, g) in garbage.into_iter().enumerate() {
            res += g.len() as i32;
            for b in g.bytes() {
                last[usize::from(b - b'G')] = i;
            }
        }
        let travel: Vec<i32> = travel
            .into_iter()
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect();
        for i in [b'G', b'M', b'P'].into_iter().map(|b| usize::from(b - b'G')) {
            if last[i] > 0 {
                res += travel[last[i] - 1];
            }
        }
        res
    }
}
