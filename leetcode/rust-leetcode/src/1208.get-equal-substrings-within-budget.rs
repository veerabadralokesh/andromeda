impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let l = s.len();
        let sb = s.into_bytes().into_iter().map(|b| b as i32).collect::<Vec<i32>>();
        let tb = t.into_bytes().into_iter().map(|b| b as i32).collect::<Vec<i32>>();
        let mut diff = vec![0; l];
        for i in 0..l {
            diff[i] = (sb[i]-tb[i]).abs();
        }
        let (mut left, mut right, mut sum, mut max_ans, mut ans) = (0, 0, 0, 0, 0);
        while right < l {
            sum += diff[right];
            ans += 1;
            right += 1;
            while sum > max_cost && left < right {
                sum -= diff[left];
                left += 1;
                ans -= 1;
            }
            max_ans = max_ans.max(ans);
        }
        max_ans
    }
}

