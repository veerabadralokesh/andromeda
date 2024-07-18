impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut increasing = true;
        for i in 1..arr.len() {
            if arr[i] == arr[i-1] {
                return false;
            }
            if arr[i] < arr[i-1] && increasing {
                if i == 1 {
                    return false;
                }
                increasing = false;
            }
            if !increasing && arr[i] > arr[i-1] {
                return false;
            }
        }
        return !increasing;
    }
}

