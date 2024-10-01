impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut rems = vec![0; k as usize];
        for num in arr {
            rems[(((num % k) + k) % k) as usize] += 1;
        }
        if rems[0] % 2 != 0 {
            return false;
        }
        let k = k as usize;
        // println!("{:?}", rems);
        for n in 1..=k/2 {
            if rems[n] != rems[k-n] {
                return false;
            }
        }
        true
    }
}

