impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut dist = 0;
        let mut can = capacity;
        let mut i = 0;
        while i < plants.len() {
            if can >= plants[i] {
                can -= plants[i];
                i += 1;
                dist += 1;
                // println!("{dist}");
            } else {
                can = capacity;
                dist += (2 * (i as i32));
            }
        }
        dist
    }
}
