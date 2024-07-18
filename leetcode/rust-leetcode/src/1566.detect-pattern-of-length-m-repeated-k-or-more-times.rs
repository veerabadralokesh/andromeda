impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        let mut count = 0;
        for i in m..arr.len() {
            count = if arr[i] == arr[i-m] {count+1} else {0};
            if count == m * k - m {
                return true;
            }
        }
        false
    }
}

