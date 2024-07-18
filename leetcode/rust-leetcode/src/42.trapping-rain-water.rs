impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let l = height.len();
        if l < 3 {
            return 0;
        }
        let mut maxh = height[0];
        let mut areas = vec![0i32; l];
        for i in 1..l {
            let h = height[i];
            if h > maxh {
                maxh = h;
                continue;
            } else {
                areas[i] = (maxh - h);
            }
        }
        maxh = height[l-1];
        areas[l-1] = 0;
        for i in (0..l-1).rev() {
            let h = height[i];
            if h > maxh {
                maxh = h;
                areas[i] = 0;
                continue;
            } else {
                areas[i] = i32::min(maxh - h, areas[i]);
            }
        }
        areas.iter().sum()
    }
}

/*
///////////////////////////////////////////////////////////////////////////////////
*/

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut ans = vec![0; height.len()];
        let mut highest = height[0];
        for i in 0..height.len() {
            highest = highest.max(height[i]);
            ans[i] = highest;
        }

        highest = *height.last().unwrap();
        for i in (0..height.len()).rev() {
            highest = highest.max(height[i]);
            ans[i] = ans[i].min(highest);
        }
        ans.iter().zip(height.iter()).map(|(a, b)| a - b).sum()
    }
}

/*
///////////////////////////////////////////////////////////////////////////////////
*/

