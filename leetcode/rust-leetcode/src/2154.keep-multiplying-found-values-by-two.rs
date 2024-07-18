impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut counts = [0; 1001];
        for n in nums {
            counts[n as usize] += 1;
        }
        let mut  x = original as usize;
        while x < counts.len() && counts[x] > 0 {
            x *= 2;
        }
        x as _
    }
}

