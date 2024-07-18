impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut i = 0;
        let mut x = 0;
        let target = target.abs();
        while x != target {
            if x < target {
                i += 1;
                x += i;
            } else {
                let diff = (x - target);
                if diff & 1 == 0 {
                    x = target;
                    break;
                } else {
                    x = target - 1;
                }
            }
        }
        i
    }
}

