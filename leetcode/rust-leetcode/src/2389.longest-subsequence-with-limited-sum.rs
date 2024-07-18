impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        for i in 1..nums.len() {
            nums[i] += nums[i-1];
        }
        let mut ans = vec![0; queries.len()];
        for (i,q) in queries.iter().enumerate() {
            match nums.binary_search(q) {
                Ok(idx) => ans[i] = idx as i32 + 1,
                Err(idx) => ans[i] = idx as i32,
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        for i in 1..nums.len() {
            nums[i] += nums[i-1];
        }
        let mut ans = vec![0; queries.len()];
        let (mut l, mut r) = (0, nums.len()-1);
        for (i,&q) in queries.iter().enumerate() {
            (l, r) = (0, nums.len() - 1);
            while l < r {
                let mid = l + (r - l)/2;
                if nums[mid] == q {
                    l = mid+1;
                    break;
                } else if nums[mid] < q {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            ans[i] = if nums[l] > q {l as i32} else {l as i32 + 1};
        }
        ans
    }
}
