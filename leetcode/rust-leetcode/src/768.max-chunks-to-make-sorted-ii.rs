impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut new_arr = arr.into_iter().enumerate().collect::<Vec<_>>();
        new_arr.sort_by_key(|e| e.1);
        let (mut previous_sum, mut running_sum, mut total_sum, mut ans) = (0, 0, 0, 0);
        for (i, &entry) in new_arr.iter().enumerate() {
            running_sum += entry.0;
            total_sum += i;
            if running_sum + previous_sum == total_sum {
                previous_sum = total_sum;
                running_sum = 0;
                ans += 1;
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        let (mut sorted_sum, mut running_sum, mut ans) = (0, 0, 0);
        for i in 0..arr.len() {
            running_sum += arr[i];
            sorted_sum += sorted_arr[i];
            if running_sum == sorted_sum {
                ans += 1;
            }
        }
        ans
    }
}


