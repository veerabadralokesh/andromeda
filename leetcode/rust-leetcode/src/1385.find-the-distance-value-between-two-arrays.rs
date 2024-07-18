impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        let mut flags = [true; 2002];
        arr2.sort();
        let mut start = 0;
        let mut end = 0;
        let d = d as usize;
        for n in arr2 {
            let nu = (n + 1000) as usize;
            start = start.max(nu.saturating_sub(d));
            end = (nu + d + 1).min(2001);
            for i in start..end {
                flags[i] = false;
            }
        }
        arr1.into_iter().filter(|&n| flags[(n + 1000) as usize]).count() as _
    }
}

/* */

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        let mut ans = 0;
        for n1 in arr1 {
            let mut flag = true;
            for n2 in arr2.iter() {
                if (n2-n1).abs() <= d {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
        ans
    }
}

