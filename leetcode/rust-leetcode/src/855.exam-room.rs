use std::collections::BTreeSet;
struct ExamRoom {
    seats: BTreeSet<i32>,
    n: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {

    fn new(n: i32) -> Self {
        Self {
            n,
            seats: BTreeSet::new(),
        }
    }
    
    fn seat(&mut self) -> i32 {
        let (mut max_dist, mut seat, mut prev, mut dist) = (0, 0, -1, 0);
        for &cur in self.seats.iter() {
            dist = if prev == -1 {cur} else {(cur-prev)/2};
            if max_dist < dist {
                max_dist = dist;
                seat = if prev == -1 {0} else {(cur + prev)/2};
            }
            prev = cur;
        }
        if prev != -1 && self.n - 1 - prev > max_dist {
            seat = self.n-1;
        }
        self.seats.insert(seat);
        seat
    }
    
    fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

