use std::collections::{HashSet,VecDeque};
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut courses = vec![vec![]; num_courses];
        for pre in prerequisites {
            courses[pre[0] as usize].push(pre[1] as usize);
        }
        let mut ans = Vec::with_capacity(num_courses);
        let mut completed = HashSet::with_capacity(num_courses);
        let mut q = VecDeque::with_capacity(num_courses);
        for i in 0..num_courses {
            if courses[i].len() == 0 {
                ans.push(i as i32);
                completed.insert(i);
            } else {
                q.push_back(i);
            }
        }
        // let mut visited = HashSet::with_capacity(num_courses);
        let mut pre_completed = false;
        let mut changed = true;
        // while let Some(c) = q.pop_front() {
        while !q.is_empty() {
            changed = false;
            for _ in 0..q.len() {
                if let Some(c) = q.pop_front() {
                    pre_completed = true;
                    for pc in courses[c].iter() {
                        if !completed.contains(pc) {
                            pre_completed = false;
                            break;
                        }
                    }
                    if pre_completed {
                        completed.insert(c);
                        ans.push(c as i32);
                        changed = true;
                    } else {
                        // visited.insert(c);
                        q.push_back(c);
                    }
                }
            }
            if !changed {
                return vec![];
            }
        }
        ans
    }
}

/* */

// LEARN

use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut courses = vec![vec![]; num_courses];
        let mut counts = vec![0; num_courses];
        for pre in prerequisites {
            courses[pre[1] as usize].push(pre[0] as usize);
            counts[pre[0] as usize] += 1;
        }
        let mut ans = Vec::with_capacity(num_courses);
        let mut q = VecDeque::with_capacity(num_courses);
        for i in 0..num_courses {
            if counts[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(c) = q.pop_front() {
            ans.push(c as i32);
            for &nc in courses[c].iter() {
                counts[nc] -= 1;
                if counts[nc] == 0 {
                    q.push_back(nc);
                }
            }
        }
        if ans.len() == num_courses {ans} else {vec![]}
    }
}

