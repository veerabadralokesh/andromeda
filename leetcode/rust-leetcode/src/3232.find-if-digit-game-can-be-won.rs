impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let (mut single, mut double) = (0, 0);
        nums.iter().for_each(|&n| {
            if n < 10 {
                single += n;
            } else {
                double += n;
            }
        });
        single != double
    }
}

