impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut counts = vec![0; *arr.iter().max().unwrap() as usize + 1];
        let threshold = arr.len() / 4;
        let mut ans = 0;
        for &n in arr.iter() {
            counts[n as usize] += 1;
            if counts[n as usize] == threshold+1 {
                ans = n;
            }
        }
        ans
    }
}

