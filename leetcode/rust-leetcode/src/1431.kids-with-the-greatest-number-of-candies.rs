impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mx:i32 = *candies.iter().max().unwrap();
        let mut ans:Vec<bool> = Vec::with_capacity(candies.len());
        for c in candies.iter() {
            ans.push((c+extra_candies) >= mx);
        }
        ans
    }
}
