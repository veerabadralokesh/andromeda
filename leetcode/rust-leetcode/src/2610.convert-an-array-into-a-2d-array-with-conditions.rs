use std::collections::HashMap;
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map:HashMap<i32, i32> = HashMap::new();
        let mut uniq:Vec<i32> = Vec::new();
        for n in nums.iter() {
            let count = map.entry(*n).or_insert(0);
            if *count == 0 {
                uniq.push(*n);
            }
            *count += 1;
        }
        let mut ans:Vec<Vec<i32>> = Vec::with_capacity(map.len());
        let mut insert_new_row: bool = true;
        while insert_new_row {
            insert_new_row = false;
            let mut row: Vec<i32> = Vec::new();
            for n in uniq.iter() {
                let count = map.entry(*n).or_insert(0);
                if *count > 0 {
                    row.push(*n);
                    *count -= 1;
                }
                if *count > 0 {
                    insert_new_row = true;
                }
            }
            ans.push(row);
        }
        ans
    }
}


impl Solution2 {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hash: HashMap<usize, i32> = HashMap::new();
        nums.iter()
            .for_each(|&num| *hash.entry(num as usize).or_insert(0) += 1);
        let mut answer: Vec<Vec<i32>> = vec![];

        while !hash.is_empty() {
            let mut localvec = vec![];
            for pair in &hash {
                localvec.push(*pair.0 as i32);
            }
            for &local in localvec.iter() {
                if *hash.get(&(local as usize)).unwrap() as i32 == 1 {
                    hash.remove(&(local as usize));
                } else {
                    hash.entry(local as usize).and_modify(|mut e| *e -= 1);
                }
            }
        answer.push(localvec);
        }
        answer        
    }
}