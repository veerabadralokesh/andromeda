use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut map = HashMap::with_capacity(26);
        let mut set = HashSet::with_capacity(26);
        for b in word.clone().into_bytes() {
            *map.entry(b).or_insert(0) += 1;
            set.insert(b);
        }
        if set.len() == word.len() {
            return true;
        }
        // println!("{:?}", map);
        // println!("{:?}", set);
        for &x in set.iter() {
            let mut xc = *(map.get(&x).unwrap()) - 1;
            // println!("{x} {xc}");
            let mut flag = true;
            for &y in set.iter() {
                if x == y {continue;}
                if xc == 0 {
                    xc = *map.get(&y).unwrap();
                }
                flag = flag && (*(map.get(&y).unwrap()) == xc);
                if !flag {
                    break;
                }
            }
            if flag {
                return flag;
            }
        }
        false
    }
}

