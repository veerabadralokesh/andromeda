impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by_key(|p| (-p[0], p[1]));
        let mut ans = vec![];
        for p in people.iter() {
            ans.insert(p[1] as usize, p.to_vec());
        }
        ans
    }
}

