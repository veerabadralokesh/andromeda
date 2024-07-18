impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subs = Vec::new();
        let mut subset:Vec<i32> = Vec::with_capacity(nums.len());
        fn generate(subs: &mut Vec<Vec<i32>>, nums: &Vec<i32>, subset: &mut Vec<i32>, idx: usize) {
            if idx == nums.len() {
                subs.push(subset.to_vec());
                return;
            }
            generate(subs, nums, subset, idx+1);
            subset.push(nums[idx]);
            generate(subs, nums, subset, idx+1);
            subset.pop();
        }
        generate(&mut subs, &nums, &mut subset, 0);
        subs
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subs = Vec::new();
        let mut subset:Vec<i32> = Vec::with_capacity(nums.len());
        fn generate(subs: &mut Vec<Vec<i32>>, nums: &Vec<i32>, subset: &mut Vec<i32>, idx: usize, l:&usize) {
            if idx == *l {
                subs.push(subset.to_vec());
                return;
            }
            generate(subs, nums, subset, idx+1, l);
            subset.push(nums[idx]);
            generate(subs, nums, subset, idx+1, l);
            subset.pop();
        }
        generate(&mut subs, &nums, &mut subset, 0, &(nums.len()));
        subs
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subs = Vec::new();
        let mut subset:Vec<i32> = Vec::with_capacity(nums.len());
        fn generate(subs: &mut Vec<Vec<i32>>, nums: &Vec<i32>, subset: &mut Vec<i32>, idx: usize) {
            if idx == 0 {
                subs.push(subset.to_vec());
                return;
            }
            let idx = idx - 1;
            generate(subs, nums, subset, idx);
            subset.push(nums[idx]);
            generate(subs, nums, subset, idx);
            subset.pop();
        }
        generate(&mut subs, &nums, &mut subset, nums.len());
        subs
    }
}
