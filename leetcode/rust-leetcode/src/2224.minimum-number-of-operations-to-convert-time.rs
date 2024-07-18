impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        if current == correct {return 0;}
        let current: Vec<_> = current.split(":").map(|s| s.parse::<i32>().unwrap()).collect();
        let correct: Vec<_> = correct.split(":").map(|s| s.parse::<i32>().unwrap()).collect();
        let mut ans = 0;
        let mut diff = (correct[0]-current[0]) * 60 + correct[1] - current[1];
        let changes = [60, 15, 5, 1];
        let mut i = 0;
        while diff > 0 {
            if diff >= changes[i] {
                diff -= changes[i];
                ans += 1;
            } else {
                i += 1;
            }
        }
        ans
    }
}

