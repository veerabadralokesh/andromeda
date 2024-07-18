impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let l = arr2.len();
        let mut indices = [l; 1001];
        for (i, n) in arr2.into_iter().enumerate() {
            indices[n as usize] = i;
        }
        arr1.sort_by_key(|&n| (indices[n as usize], n));
        arr1
    }
}

