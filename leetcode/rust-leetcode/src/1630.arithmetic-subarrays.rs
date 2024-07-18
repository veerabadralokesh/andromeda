impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = Vec::with_capacity(nums.len());
        let l = l.iter().map(|&x| x as usize).collect::<Vec<usize>>();
        let r = r.iter().map(|&x| x as usize).collect::<Vec<usize>>();
        for i in 0..l.len() {
            if r[i]-l[i] < 2 {
                ans.push(true);
                continue;
            }
            let mut sub = (l[i]..=r[i]).map(|si| nums[si]).collect::<Vec<i32>>();
            sub.sort();
            let diff = sub[1] - sub[0];
            let mut is_arithmetic = true;
            for j in 2..sub.len() {
                if sub[j]-sub[j-1] != diff {
                    is_arithmetic = false;
                    break;
                }
            }
            ans.push(is_arithmetic);
        }
        ans
    }
}
