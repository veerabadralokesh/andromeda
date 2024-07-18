impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let num_people = num_people as usize;
        let mut people = vec![0; num_people];
        let mut i = 0;
        let mut count = 1;
        loop {
            i = 0;
            while candies > 0 && i < num_people {
                people[i] += count.min(candies);
                candies -= count;
                count += 1;
                i += 1;
            }
            if candies < 1 {
                break;
            }
        }
        people
    }
}

