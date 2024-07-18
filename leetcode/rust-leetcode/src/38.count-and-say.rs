impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut rle = Vec::with_capacity(20000);
        rle.push(1);
        let mut i = 0;
        let mut count = 0;
        let mut num = 0;
        let mut end = rle.len();
        for _ in 1..n {
            end = rle.len();
            num = rle[i];
            count = 1;
            for j in i+1..end {
                if rle[j] != num {
                    rle.push(count);
                    rle.push(num);
                    num = rle[j];
                    count = 1;
                } else {
                    count += 1;
                }
            }
            rle.push(count);
            rle.push(num);
            i = end;
            // println!("{:?}", rle[i..].to_vec());
        }
        // println!("{}", rle.len());
        rle[i..].into_iter().map(|&n| (n as u8 + b'0') as char).collect()
    }
}

