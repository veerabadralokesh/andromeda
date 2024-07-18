impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut ans = vec![1i32; ratings.len()];
        let mut prev_r = 0i32;
        let mut prev_c = 0i32;
        let mut r = 0i32;
        let mut c = 0i32;
        for i in 0..ratings.len() {
            r = ratings[i];
            if r > prev_r {
                ans[i] = prev_c + 1;
            }
            prev_r = r;
            prev_c = ans[i];
        }
        // println!("{:?}", ans);
        prev_r = 0;
        prev_c = 0;
        for i in (0..ratings.len()).rev() {
            r = ratings[i];
            // println!("{r}, {prev_r}, {prev_c}, {}", ans[i]);
            if r > prev_r {
                ans[i] = ans[i].max(prev_c + 1);
            }
            prev_r = r;
            prev_c = ans[i];
        }
        // println!("{:?}", ans);
        ans.iter().sum()
    }
}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1i32; n];
        for i in 1..n {
            if ratings[i-1] < ratings[i] {
                candies[i] = candies[i-1] + 1;
            }
        }
        for i in (0..(n-1)).rev() {
            if (ratings[i] > ratings[i+1]) && (candies[i] <= candies[i+1]) {
            candies[i] = candies[i+1] + 1;
            }
        }
        candies.iter().sum()
    }
}
