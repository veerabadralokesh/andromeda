impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let time = time % (2 * n - 2);
        if time < n - 1 {
            time + 1
        } else {
            2 * n - time - 1
        }
    }
}

