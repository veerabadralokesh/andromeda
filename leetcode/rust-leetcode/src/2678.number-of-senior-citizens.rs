impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut ans = 0;
        for d in details.iter() {
            // details.iter().map(|&d| ).filter(|a| a.unwrap() > 60).count() as i32
            // let a = d.chars().collect::<Vec<_>>()[11..13].into_iter().collect::<String>().parse::<i32>();
            if d.chars().collect::<Vec<_>>()[11..13].into_iter().collect::<String>().parse::<i32>().unwrap() > 60 {
                ans += 1;
            }
        }
        ans
    }
}

