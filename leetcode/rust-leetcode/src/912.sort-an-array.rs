impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn quick_sort(nums: &mut [i32]) {
            if nums.len() < 2 {
                return;
            }
            let l = nums.len();
            let (mut i, mut j) = (0, 0);
            let pivot = nums[l-1];
            for j in 0..l-1 {
                if nums[j] < pivot {
                    nums.swap(i, j);
                    i += 1;
                }
            }
            nums.swap(i, l-1);
            quick_sort(&mut nums[..i]);
            j = i + 1;
            while j < l && nums[j] == nums[i] {
                j += 1;
            }
            quick_sort(&mut nums[j..]);
        }
        quick_sort(&mut nums);
        nums
    }
}

/* */

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 100001];
        nums.iter().for_each(|n| {
            counts[(*n + 50000) as usize] += 1;
        });
        let mut ans = Vec::with_capacity(nums.len());
        for (i, count) in counts.iter().enumerate() {
            for _ in 0..*count {
                ans.push(i as i32 - 50000);
            }
        }
        ans
    }
}


