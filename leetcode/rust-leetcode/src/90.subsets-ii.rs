use std::collections::HashSet;
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut subs = HashSet::new();
        let mut subset:Vec<i32> = Vec::with_capacity(nums.len());
        fn generate(subs: &mut HashSet<Vec<i32>>, nums: &Vec<i32>, subset: &mut Vec<i32>, idx: usize) {
            if idx == 0 {
                subs.insert(subset.to_vec());
                return;
            }
            let idx = idx - 1;
            generate(subs, nums, subset, idx);
            subset.push(nums[idx]);
            generate(subs, nums, subset, idx);
            subset.pop();
        }
        generate(&mut subs, &nums, &mut subset, nums.len());
        subs.into_iter().collect()
    }
}
