impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        nums.sort();
        let valid_distance = |d: i32| -> bool {
            let mut count = 0;
            let (mut i, mut j) = (0, 0);
            while i < n || j < n {
                while j < n && nums[j] - nums[i] <= d {
                    j += 1;
                }
                count += j - i - 1;
                i += 1;
            }
            count >= k
        };

        let (mut start, mut end) = (0, nums[n-1]-nums[0]);
        while start < end {
            let mid = start + (end - start)/2;
            if valid_distance(mid) {
                end = mid;
            } else {
                start = mid+1;
            }
        }
        start
    }
}

