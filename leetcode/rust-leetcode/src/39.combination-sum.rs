use std::collections::{VecDeque,HashSet};
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut ans:HashSet<Vec<i32>> = HashSet::new();
        for i in 0..candidates.len() {
            if candidates[i] == target {
                ans.insert(vec![candidates[i]]);
            } else if candidates[i] < target {
                queue.push_back((target-candidates[i], vec![candidates[i]]));
            }
        }
        while let Some((remaining, combo)) = queue.pop_front() {
            for i in 0..candidates.len() {
                if candidates[i] == remaining {
                    let mut combo = combo.to_vec();
                    combo.push(candidates[i]);
                    combo.sort();
                    ans.insert(combo);
                } else if candidates[i] < remaining {
                    // map.insert(target-candidates[i], vec![candidates[i]]);
                    let mut combo = combo.to_vec();
                    combo.push(candidates[i]);
                    queue.push_back((remaining-candidates[i], combo));
                }
            }
        }
        ans.into_iter().collect()
    }
}

/* */

impl Solution {

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current: Vec<i32> = Vec::new();

        fn backtrack(result: &mut Vec<Vec<i32>>, candidates: &[i32], target: i32, current: &mut Vec<i32>) {
            if target == 0 {
                result.push(current.to_vec());
                return;
            }
            if target < 0 {
                return;
            }
            for (i, &candidate) in candidates.iter().enumerate() {
                current.push(candidate);
                backtrack(result, &candidates[i..], target - candidate, current );
                current.pop();
            }
        }

        backtrack(&mut result, &candidates, target, &mut current);
        result
    }
}
