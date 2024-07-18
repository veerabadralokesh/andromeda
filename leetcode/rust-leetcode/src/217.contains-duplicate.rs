use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != nums.iter().collect::<HashSet<_>>().len()
    }
}

/* */

// LEARN

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        nums.into_iter().any(|n| !set.insert(n))
    }
}
