impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap() as usize;
        let mut counts = vec![0; mx+1];
        for &n in nums.iter() {
            counts[n as usize] += 1;
        }
        let mut scores = vec![0; mx + 1];
        scores[1] = counts[1];
        for i in 2..=mx {
            scores[i] = scores[i-1].max(scores[i-2] + counts[i] * i);
        }
        scores[mx] as _
    }
}

