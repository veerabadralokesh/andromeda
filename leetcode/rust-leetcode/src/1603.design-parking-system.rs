struct ParkingSystem {
    counts: [i32; 3],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            counts: [big, medium, small],
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        match self.counts[car_type as usize - 1] {
            b if b > 0 => {
                self.counts[car_type as usize - 1] -= 1;
                true
            },
            _ => false,
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
