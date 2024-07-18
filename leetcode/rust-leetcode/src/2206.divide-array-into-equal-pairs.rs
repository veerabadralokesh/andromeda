impl Solution {
    pub fn divide_array(mut nums: Vec<i32>) -> bool {
        let mut counts = [0; 501];
        for n in nums {
            counts[n as usize] += 1;
        }
        counts.iter().all(|x| x % 2 == 0)
    }
}
