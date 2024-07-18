impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        // println!("{:?}", n.bytes());
        n.bytes().map(|c| c - 48).max().unwrap() as i32
    }
}

impl Solution2 {
    pub fn min_partitions(n: String) -> i32 {
        let mut max:u32 = 0;
        for num in n.chars().map(|x| x.to_digit(10)) {
            max = Ord::max(max, num.unwrap());
        }
        max as i32
    }
}
