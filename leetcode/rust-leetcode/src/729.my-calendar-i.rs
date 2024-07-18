use std::collections::BTreeSet;
struct MyCalendar {
    cal: BTreeSet<(i32, i32)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self {cal: BTreeSet::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        if !self.cal.is_empty() && (end > self.cal.first().unwrap().0 || start < self.cal.last().unwrap().1) {
            for &(st, ed) in self.cal.iter() {
                if start >= st && start < ed {
                    // println!("1 [{start},{end}]");
                    return false;
                }
                if end > st && end <= ed {
                    // println!("2 [{start},{end}]");
                    return false;
                }
                if start >= st && end <= ed {
                    // println!("3 [{start},{end}]");
                    return false;
                }
                if st >= start && ed <= end {
                    // println!("3.1 [{start},{end}]");
                    return false;
                }
            }
        }
        // println!("4 [{start},{end}]");
        // println!("{:?}", self.cal);
        self.cal.insert((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

/* */

// LEARN

struct MyCalendar {
    buffer: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self {
            buffer: std::collections::BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, last_end)) = self.buffer.range(..end).last() {
            if start < *last_end {
                return false;
            }
        }
        self.buffer.insert(start, end);
        true
    }
}


