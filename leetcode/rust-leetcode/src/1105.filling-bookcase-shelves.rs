impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![i32::MAX; books.len()+1];
        dp[0] = 0;
        let (mut t_sum, mut thickness, mut max_height, mut h) = (0, 0, 0, 0);
        for i in 0..books.len() {
            (t_sum, max_height) = (0, 0);
            for j in (0..=i).rev() {
                (thickness, h) = (books[j][0], books[j][1]);
                t_sum += thickness;
                if t_sum > shelf_width {
                    break;
                }
                max_height = max_height.max(h);
                dp[i+1] = dp[i+1].min(dp[j] + max_height)
            }
        }
        dp[books.len()]
    }
}

/* */

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let l = books.len();
        let mut dp = vec![i32::MAX; l+1];
        dp[l] = 0;
        let (mut max_h, mut thickness_sum, mut height, mut thickness) = (0, 0, 0, 0);
        for i in (0..l).rev() {
            (max_h, thickness_sum) = (0, 0);
            for j in i..l {
                (thickness, height) = (books[j][0], books[j][1]);
                thickness_sum += thickness;
                if thickness_sum > shelf_width {
                    break;
                }
                max_h = max_h.max(height);
                dp[i] = dp[i].min(dp[j+1] + max_h);
            }
        }
        dp[0]
    }
}

