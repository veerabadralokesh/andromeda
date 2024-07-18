use std::collections::VecDeque;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students.into_iter().collect::<VecDeque<i32>>();
        let mut sandwiches = sandwiches.into_iter().collect::<VecDeque<i32>>();
        let mut to_eat = students.len();
        let mut rotate = 0;
        while to_eat > 0 && rotate < to_eat {
            if students[0] == sandwiches[0] {
                students.pop_front();
                sandwiches.pop_front();
                rotate = 0;
                to_eat -= 1;
            } else {
                students.rotate_right(1);
                rotate += 1;
            }
        }
        to_eat as i32
    }
}
