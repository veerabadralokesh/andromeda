use std::collections::HashMap;
struct OrderedStream {
    map: HashMap<i32,String>,
    ptr: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        Self {
            map: HashMap::new(),
            ptr: 1,
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.map.insert(id_key, value);
        let mut chunk = Vec::new();
        if id_key == self.ptr {
            while self.map.contains_key(&self.ptr) {
                chunk.push(self.map.get(&self.ptr).unwrap().clone());
                self.ptr += 1;
            }
        }
        chunk
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

/* */

struct OrderedStream {
    items: Vec<Option<String>>,
    ptr: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self { items: vec![None; n as usize], ptr: 0 }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.items[id_key as usize - 1] = Some(value);

        let mut ans = vec![];

        for item in self.items.iter().skip(self.ptr) {
            if let Some(val) = item {
                ans.push(val.to_string());
                self.ptr += 1;
            } else {
                break;
            }
        }
        ans
    }
}


