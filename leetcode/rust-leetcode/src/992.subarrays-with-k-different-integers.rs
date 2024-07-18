impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn subarrays_with_atmost_k(nums: Vec<i32>, k: i32) -> i32 {
            let mut counts = [0u16;20001];
            let mut ans=0;
            let mut start=0;
            let mut count=0;
            for i in (0..nums.len()){
                counts[nums[i] as usize] += 1;
                if counts[nums[i] as usize] == 1 {
                    count += 1;
                }
                while count > k {
                    counts[nums[start] as usize] -= 1;
                    if counts[nums[start] as usize] == 0 {
                        count-=1;
                    }
                    start+=1;
                }
                ans += (i - start + 1);
            }
            ans as i32
        }
        subarrays_with_atmost_k(nums.clone(), k) - subarrays_with_atmost_k(nums, k-1)
    }
}

/* */

// LEARN

impl Solution {
    // subarray
    // exactly k distinct
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n : usize = nums.len();
        let mut freq1: Vec<u32> = vec![0; n + 1];
        let mut freq2: Vec<u32> = vec![0; n + 1];
        let (mut r, mut l1, mut l2) : (usize, usize, usize) = (0, 0, 0);
        let (mut c1, mut c2) : (usize, usize) = (0, 0);
        let mut ans: i32 = 0;
        for r in 0..n {
            let num = nums[r] as usize;
            c1 += (freq1[num] == 0) as usize;
            c2 += (freq2[num] == 0) as usize;
            freq1[num] += 1;
            freq2[num] += 1;
            // println!{"r is {}, c1 is {}, c2 is {}", r, c1, c2};
            while (l2 <= r && c2 > k - 1) {
                let num = nums[l2] as usize;
                freq2[num] -= 1;
                c2 -= (freq2[num] == 0) as usize;
                l2 += 1;
            }
            while (l1 <= r && c1 > k) {
                let num = nums[l1] as usize;
                freq1[num] -= 1;
                c1 -= (freq1[num] == 0) as usize;
                l1 += 1;
            }
            if (c1 == k) {
                ans += (l2 - l1) as i32;
                // println!{"l1 is {}, l2 is {}", l1, l2};
            }
        }
        return ans;
    }
}
