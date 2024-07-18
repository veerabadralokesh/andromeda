use std::collections::HashMap;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map1:HashMap<i32, i32> = HashMap::new();
        let mut map2:HashMap<i32, i32> = HashMap::new();
        let mut ans:Vec<Vec<i32>> = Vec::with_capacity(2);
        for n in nums1.iter() {
            *map1.entry(*n).or_insert(0) += 1;
            map2.entry(*n).or_insert(0);
        }
        for n in nums2.iter() {
            *map2.entry(*n).or_insert(0) += 1;
            map1.entry(*n).or_insert(0);
        }
        map1.retain(|_, v| *v == 0);
        map2.retain(|_, v| *v == 0);
        ans.push(map2.keys().copied().collect::<Vec<_>>());
        ans.push(map1.keys().copied().collect::<Vec<_>>());
        // println!("{:?}", ans);
        ans
    }
}

use std::{collections::{hash_map::RandomState, HashSet}, hash::BuildHasher};
use std::iter::FromIterator;

impl Solution2 {

    fn f<T: BuildHasher>(setA: &HashSet<i32, T>, setB: &HashSet<i32, T>) -> Vec<i32>{
        let mut temp = Vec::new();
        for x in setA.iter() {
            if !setB.contains(x) {
                temp.push(*x);
            }
        }
        temp
    }

    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32, RandomState> = HashSet::from_iter(nums1);
        let set2: HashSet<i32, RandomState> = HashSet::from_iter(nums2);

        let ans1 = Self::f(&set1, &set2);
        let ans2 = Self::f(&set2, &set1);

        vec![ans1, ans2]
    }
}

impl Solution3 {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let idx = |x: i32| -> usize { (x + 1000) as _ };
        let mut counter = [0_u8; 2001];
        nums1.iter().for_each(|&x| counter[idx(x)] |= 0b01);
        nums2.iter().for_each(|&x| counter[idx(x)] |= 0b10);

        let mut res = vec![Vec::new(), Vec::new()];
        counter.iter().enumerate().for_each(|(i, &x)| match x {
            0b01 => res[0].push(i as i32 - 1000),
            0b10 => res[1].push(i as i32 - 1000),
            _ => (),
        });
        res
    }
}

impl Solution4 {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left_map = [false; 2001];
        let mut right_map = [false; 2001]; 

        for n in nums1 {
            let idx = n + 1000;
            left_map[idx as usize] = true;
        }
        for n in nums2 {
            let idx = n + 1000;
            right_map[idx as usize] = true;
        }

        let only_left = Vec::new();
        let only_right = Vec::new();
        let mut result = vec![only_left, only_right];
        for (i, (left, right)) in left_map.into_iter().zip(right_map).enumerate() {
            // 0 => neither
            // 1 => only_left
            // 2 => only_right
            // 3 => both
            let idx = *left as u8 + 2 * right as u8;
            let idx = idx - 1;
            if let Some(vec) = result.get_mut(idx as usize) {
                let original_number = i as i32 - 1000;
                vec.push(original_number);
            }
        }
        
        result
    }
}