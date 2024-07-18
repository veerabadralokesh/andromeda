impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut count = 0;
        let mut narr = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            narr.push(arr[i]);
            if narr.len() == arr.len() {
                break;
            }
            if arr[i] == 0 {
                narr.push(0);
            }
            if narr.len() == arr.len() {
                break;
            }
        }
        for i in 0..arr.len() {
            arr[i] = narr[i];
        }
    }
}

