use std::{cmp::Reverse, collections::BinaryHeap};
struct SeatManager {
    seats: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        let mut seats = BinaryHeap::new();
        // for i in 1..n+1 {
        //     seats.push(Reverse(i));
        // }
        seats.push(Reverse(1));
        Self {seats}
    }
    
    fn reserve(&mut self) -> i32 {
        let seat = self.seats.pop().unwrap().0;
        if self.seats.len() == 0 {
            self.seats.push(Reverse(seat+1));
        }
        seat
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(Reverse(seat_number));
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
