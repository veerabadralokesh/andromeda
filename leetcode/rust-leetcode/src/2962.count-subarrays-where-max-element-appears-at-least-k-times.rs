impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_elem = nums.clone().into_iter().max().unwrap();
        let mut ans = 0usize;
        let mut max_elem_indices = Vec::new();
        let k = k as usize;
        for (i, n) in nums.into_iter().enumerate() {
            if n == max_elem {
                max_elem_indices.push(i);
            }
            if max_elem_indices.len() >= k {
                ans += max_elem_indices[max_elem_indices.len()-k] + 1;
            }
        }
        ans as i64
    }
}

/* */

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_elem = *nums.iter().max().unwrap();
        let mut ans = 0usize;
        let mut start = 0usize;
        let mut end = 0usize;
        let mut count = 0i32;
        while end < nums.len() {
            if nums[end] == max_elem {
                count += 1;
            }
            while count >= k {
                ans += (nums.len() - end);
                if nums[start] == max_elem {
                    count -= 1;
                }
                start += 1;
            }
            end += 1;
        }
        ans as i64
    }
}

/* */

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans=0i64;
        let max=*nums.iter().max().unwrap();
        let mut start=0;
        let mut count=0;
        for i in (0..nums.len()){
            if nums[i]==max{
                count+=1;
            }
            while count>=k{
                ans+=(nums.len()-i) as i64;
                if nums[start]==max{
                    count-=1;
                }
                start+=1;
            }
        }
        return ans
    }
}
