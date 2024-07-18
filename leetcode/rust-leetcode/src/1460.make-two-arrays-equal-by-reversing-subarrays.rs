impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        if target.len() != arr.len() {
            return false;
        }
        let mut counts = [0; 1001];
        for t in target {
            counts [t as usize] += 1;
        }
        for a in arr {
            if counts[a as usize] == 0 {
                return false;
            }
            counts[a as usize] -= 1;
        }
        true
    }
}

