impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut names = names;
        let mut indices = (0..names.len()).collect::<Vec<usize>>();
        indices.sort_by_key(|i| heights[*i]);
        indices.reverse();
        names = indices.into_iter().map(|x| names[x].clone()).collect::<Vec<String>>();
        names
    }
}

use std::cmp::Reverse;
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zipped: Vec<_> = heights.into_iter().zip(names.into_iter()).collect();
        zipped.sort_unstable_by_key(|(h, n)| Reverse(*h));
        zipped.into_iter().map(|(_, n)| n).collect()
    }
}

