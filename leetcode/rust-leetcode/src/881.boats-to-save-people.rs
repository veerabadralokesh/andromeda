impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let (mut small, mut big) = (0, people.len() - 1);
        let mut boats = 0;
        while small < big {
            if people[small] + people[big] <= limit {
                small += 1;
            }
            big -= 1;
            boats += 1;
        }
        if small == big {
            boats += 1;
        }
        boats
    }
}
