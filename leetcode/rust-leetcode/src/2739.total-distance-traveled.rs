impl Solution {
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut dist = 0;
        while main_tank > 0 {
            if main_tank > 4 && additional_tank > 0 {
                main_tank -= 4;
                additional_tank -= 1;
                dist += 50;
            } else {
                dist += main_tank * 10;
                break;
            }
        }
        dist
    }
}

