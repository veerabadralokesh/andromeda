impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates.into_iter().filter(|&c| c <= target).collect::<Vec<_>>();
        candidates.sort();
        fn backtrack(ans: &mut Vec<Vec<i32>>, target: i32, vec: &mut Vec<i32>, candidates: &Vec<i32>, i: usize) {
            if target < 0 {
                return;
            }
            if target == 0 {
                ans.push(vec.to_vec());
                return;
            }
            for j in i..candidates.len() {
                if j > i && candidates[j] == candidates[j-1] {
                    continue;
                }
                vec.push(candidates[j]);
                backtrack(ans, target-candidates[j], vec, candidates, j+1);
                vec.pop();
            }
        }
        let mut ans = vec![];
        let mut vec = vec![];
        backtrack(&mut ans, target, &mut vec, &candidates, 0);
        ans
    }
}


