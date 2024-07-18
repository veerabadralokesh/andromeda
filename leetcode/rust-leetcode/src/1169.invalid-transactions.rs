use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        for (i,t) in transactions.iter().enumerate() {
            if let &[name, time, amount, city] = t.split(',').collect::<Vec<_>>().as_slice() {
                let amount = amount.parse::<u32>().unwrap();
                if amount > 1000 {
                    set.insert(i);
                }
                map.entry(name).or_insert(Vec::new()).push((time.parse::<i32>().unwrap(), city, i));
            }
        }
        for (k, mut v) in map.into_iter() {
            v.sort();
            let mut i = 0;
            while i < v.len()-1 {
                for j in i+1..v.len() {
                    if v[j].0-v[i].0 <= 60 && v[j].1 != v[i].1 {
                        set.insert(v[i].2);
                        set.insert(v[j].2);
                    }
                }
                i += 1;
            }
        }
        (0..transactions.len()).filter(|i| set.contains(i)).map(|i| transactions[i].clone()).collect::<Vec<_>>()
    }
}

