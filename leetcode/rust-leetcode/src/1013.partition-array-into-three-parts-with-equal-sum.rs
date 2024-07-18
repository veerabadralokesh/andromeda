impl Solution {
    pub fn can_three_parts_equal_sum(mut arr: Vec<i32>) -> bool {
        let sum = arr.iter().sum::<i32>();
        // println!("{:?} {:?}", partition, arr);
        if sum % 3 != 0 {
            return false;
        }
        let partition = sum/3;
        let mut part_count = 0;
        let mut psum = 0;
        for k in 0..arr.len() {
            psum += arr[k];
            if psum == partition {
                psum = 0;
                part_count += 1;
            }
        }
        part_count >= 3
    }
}

