impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ge = -1;
        for i in (0..arr.len()).rev() {
            let temp = arr[i];
            arr[i] = ge;
            ge = ge.max(temp);
        }
        arr
    }
}

/* */

// LEARN

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        arr.into_iter()
            .rev()
            .scan(-1, |acc, x| {
                let tmp = Some(*acc);
                *acc = (*acc).max(x);
                tmp
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .collect()
    }
}

