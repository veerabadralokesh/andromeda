impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let n = satisfaction.len();
        satisfaction.sort();
        if satisfaction[n-1] < 0 {return 0;}
        let mut liketime = 0i32;
        let mut sumsofar = 0i32;
        for s in satisfaction.iter().rev() {
            sumsofar += s;
            if sumsofar > 0 {
                liketime += sumsofar;
            } else {
                break;
            }
        }
        liketime
    }
}
