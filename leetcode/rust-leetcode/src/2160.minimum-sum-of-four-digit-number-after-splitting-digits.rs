impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut n = num.to_string().bytes().map(|b| (b-48) as i32).collect::<Vec<i32>>();
        // println!("{:?}", n);
        n.sort();
        n[3] + n[2] + n[1] * 10 + n[0] * 10
    }
}
