impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut ans = 0;
        let l = colors.len();
        for i in 0..l-2 {
            if colors[i] == colors[i+2] && colors[i] != colors[i+1] {
                ans += 1;
            }
        }
        for i in colors.len()-2..colors.len() {
            if colors[i] == colors[(i+2)%l] && colors[i] != colors[(i+1)%l] {
                ans += 1;
            }
        }
        ans
    }
}

