impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score.to_vec();
        score.sort_by_key(|s| s[k as usize]);
        score.reverse();
        score
    }
}

impl Solution2 {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // let mut score = score.to_vec();
        let k = k as usize;
        score.sort_by_key(|s| -s[k]);
        // score.reverse();
        score
    }
}
impl Solution3 {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_by(|v0, v1| v1[k as usize].cmp(&v0[k as usize]));

        score
    }
}