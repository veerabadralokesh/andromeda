impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = (1..n+1).collect::<Vec<i32>>();
        let mut out:usize = 0;
        while friends.len() > 1 {
            let n = friends.len();
            out = (out + (k as usize) - 1) % n;
            friends.remove(out);
            // println!("{:?}", friends);
        }
        friends[0]
    }
}


/*
*/

use std::collections::VecDeque;

impl Solution2 {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
         let mut queue: VecDeque<i32> = (1..(n+1)).collect();
        //  println!("{:?}", queue);
         while queue.len() > 1 {
             queue.rotate_left((k-1) as usize % queue.len());
             queue.pop_front();
         }
         return queue.pop_front().unwrap();
    }
}