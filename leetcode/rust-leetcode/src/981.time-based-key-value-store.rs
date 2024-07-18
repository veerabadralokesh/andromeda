use std::collections::HashMap;
struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap{
            map: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        (*self.map.entry(key).or_insert(Vec::new())).push((value, timestamp));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        let mut ans = "".to_string();
        match self.map.get(&key) {
            None => ans,
            Some(v) => {
                if timestamp < v[0].1 {return ans;}
                let mut r = v.len()-1;
                if timestamp >= v[r].1 {return v[r].0.clone();}
                let mut l = 0usize;
                while l <= r {
                    let mid = l + (r-l)/2;
                    if v[mid].1 <= timestamp {
                        ans = v[mid].0.clone();
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
                ans
            }
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
