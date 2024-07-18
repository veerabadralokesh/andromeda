impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let s = num.to_string();
        let sc = s.clone().chars().collect::<Vec<_>>();
        let min = s.replace(sc[0], "0").parse::<i32>().unwrap();
        let mut i = 0;
        for j in 0..s.len() {
            if sc[j] != '9' {
                i = j;
                break;
            }
        }
        let max = s.replace(sc[i], "9").parse::<i32>().unwrap();
        max - min
    }
}

