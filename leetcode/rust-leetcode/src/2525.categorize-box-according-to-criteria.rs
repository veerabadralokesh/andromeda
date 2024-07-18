const MAX_MASS: i32 = 100;
const MAX_DIM: i32 = 10_000;
const MAX_VOL: i64 = 1_000_000_000;
impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let is_bulky = length >= MAX_DIM || width >= MAX_DIM || height >= MAX_DIM
                        || (length as i64) * (width as i64) * (height as i64) >= MAX_VOL;
        let is_heavy = mass >= MAX_MASS;
        match (is_bulky, is_heavy) {
            (true, true) => String::from("Both"),
            (false, true) => String::from("Heavy"),
            (true, false) => String::from("Bulky"),
            _ => String::from("Neither"),
        }
    }
}

