impl Solution {
    pub fn three_consecutive_odds(mut arr: Vec<i32>) -> bool {
        for i in 0..arr.len() {
            if arr[i] & 1 == 0 {
                arr[i] = 0;
            } else {
                arr[i] = if i > 0 {arr[i-1] + 1} else {1};
                if arr[i] == 3 {
                    return true;
                }
            }
        }
        false
    }
}

