impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let (mut ans, mut left_less, mut right_less, mut left_more, mut right_more) = (0, 0, 0, 0, 0);
        for i in 1..rating.len() {
            (left_less, right_less, left_more, right_more) = (0, 0, 0, 0);
            for j in 0..i {
                if rating[i] < rating[j] {
                    left_more += 1;
                } else {
                    left_less += 1;
                }
            }
            for j in i+1..rating.len() {
                if rating[i] < rating[j] {
                    right_more += 1;
                } else {
                    right_less += 1;
                }
            }
            ans += (left_less * right_more + left_more * right_less);
        }
        ans
    }
}

