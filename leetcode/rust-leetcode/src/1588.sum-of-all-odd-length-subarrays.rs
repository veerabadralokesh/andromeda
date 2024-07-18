impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut s:i32 = arr.clone().into_iter().sum();
        // println!("{s}");
        for l in (3..arr.len()+1).step_by(2) {
            for i in (0..(arr.len()-l+1)) {
                s += arr[i..i+l].to_vec().iter().sum::<i32>();
            }
        }
        s
    }
}

impl Solution2 {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        for index in (1..=arr.len()).step_by(2) {
            sum += arr.windows(index).flatten().sum::<i32>();
        }
        sum
    }
}
