use std::collections::{HashMap,HashSet,VecDeque};
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut courses = HashMap::with_capacity(num_courses as usize);
        for pre in prerequisites {
            courses.entry(pre[0]).or_insert(HashSet::new()).insert(pre[1]);
        }
        let mut completed_courses = HashSet::new();
        for c in 0..num_courses {
            let mut course_set = HashSet::new();
            let mut q = VecDeque::new();
            q.push_back(c);
            while let Some(course) = q.pop_front() {
                course_set.insert(course);
                completed_courses.insert(course);
                match courses.get(&course) {
                    None => {},
                    Some(preset) => {
                        for &preq in preset.into_iter() {
                            if preq == c {
                                return false;
                            }
                            if !course_set.contains(&preq) {
                                q.push_back(preq);
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
