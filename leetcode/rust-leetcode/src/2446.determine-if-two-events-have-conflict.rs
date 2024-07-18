impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let parse_time = |t: &String| -> i32 {
            match t.split_once(':') {
                Some((h, m)) => {
                    h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()
                },
                _ => unreachable!()
            }
        };
        let e1 = event1.iter().map(|s| parse_time(s)).collect::<Vec<_>>();
        let e2 = event2.iter().map(|s| parse_time(s)).collect::<Vec<_>>();
        (e1[0] <= e2[0] && e1[1] >= e2[0]) || (e2[0] <= e1[0] && e2[1] >= e1[0])
    }
}

