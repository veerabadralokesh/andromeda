impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = [0; 1001];
        let l = nums.len();
        for arr in nums.iter() {
            for &n in arr.iter() {
                counts[n as usize] += 1;
            }
        }
        let mut ans = Vec::new();
        for i in 1..1001 {
            if counts[i] == l {
                ans.push(i as i32);
            }
        }
        ans
    }
}

