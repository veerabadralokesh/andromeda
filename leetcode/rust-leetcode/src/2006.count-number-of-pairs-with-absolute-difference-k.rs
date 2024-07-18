use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut map:HashMap<i32, i32> = HashMap::new();
        let mut uniq:Vec<i32> = Vec::new();
        for n in nums.iter() {
            let count = map.entry(*n).or_insert(0);
            if (*count == 0) {
                uniq.push(*n);
            }
            *count += 1;
        }
        // println!("{:?}", map);
        // println!("{:?}", uniq);
        uniq.sort();
        // uniq.reverse();
        let mut count:i32 = 0;
        for n in uniq.iter() {
            // let m = n+k;
            let c1 = map.get(&(n+k)).copied().unwrap_or(0);
            let c2 = map.get(n).copied().unwrap_or(0);
            count += c1 * c2;
            // println!("{:?}, {}, {}, {}, {}, {}", map, count,c1, c2, m, n);
            // map.insert(*n, 0);
        }
        count
    }
}

impl Solution2 {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        // key stores element of num, value stores number of occurences of element
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut pairs: i32 = 0;

        for num in nums.iter() {
            let num1 = num + k;
            let num2 = num - k;
            if let Some(&res) = map.get(&num1) {
                pairs += res;
            }
            
            if let Some(&res) = map.get(&num2) {
                pairs += res;
            } 

            map.entry(*num).and_modify(|res| *res += 1).or_insert(1);
        }

        pairs
    }
}
