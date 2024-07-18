impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counts = [0; 501];
        for i in arr {
            counts[i as usize] += 1;
        }
        for i in (1..501).rev() {
            if counts[i] == (i as i32) {
                return i as i32;
            }
        }
        -1
    }
}
