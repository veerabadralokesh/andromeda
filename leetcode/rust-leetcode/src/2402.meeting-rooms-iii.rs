
// TODO

// use std::collections::HashMap;
// use std::collections::BinaryHeap;
// use std::cmp::Reverse;


impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut rooms:Vec<i64> = vec![0; n];
        let mut counts:Vec<i32> = vec![0; n];
        // let mut heap:BinaryHeap<i32> = BinaryHeap::new();
        // let mut m_end_room:HashMap<i32,Vec<i32>> = HashMap::with_capacity(n);
        // for i in 0..n {*m_end_room.entry(0).or_insert(Vec<i32>::new()).push(i as i32);}

        meetings.sort_by_key(|m| m[0]);

        let mut min_index:usize = 0;

        // println!("{:?}", meetings);
        // println!("{:?}", meetings[0]);
        for m in meetings.iter() {
            // println!("{:?}", m);
            min_index = 0;
            for i in 1..n {
                // println!("{:?}", rooms);
                if rooms[min_index] <= (m[0] as i64) {
                    break;
                }
                if rooms[min_index] > rooms[i] {
                    min_index = i;
                }
            }
            counts[min_index] += 1;
            rooms[min_index] = rooms[min_index].max(m[0] as i64) + ((m[1]-m[0]) as i64);
            // println!("{:?}", rooms);
        }
        // let x:i32 = *counts.iter().max().unwrap();
        // println!("{:?}",counts);
        // println!("{:?}",rooms);
        let mut x:usize = 0;
        for i in 1..n {
            if counts[i] > counts[x] {
                x = i;
            }
        }
        x as i32
    }
}

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        meetings.sort_unstable();

        let mut meeting_count_by_room = vec![0usize; n];
        let mut busy_rooms: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut free_rooms = (0..n).map(Reverse).collect::<BinaryHeap<_>>();

        for m in meetings {
            let start = m[0] as i64;
            let stop = m[1] as i64;

            while busy_rooms.peek().is_some() && busy_rooms.peek().unwrap().0 .0 <= start {
                let Reverse((_, room)) = busy_rooms.pop().unwrap();
                free_rooms.push(Reverse(room));
            }

            if let Some(Reverse(room)) = free_rooms.pop() {
                busy_rooms.push(Reverse((stop, room)));
                meeting_count_by_room[room] += 1;
            } else if let Some(Reverse((end, room))) = busy_rooms.pop() {
                busy_rooms.push(Reverse((end + stop - start, room)));
                meeting_count_by_room[room] += 1;
            }
        }

        meeting_count_by_room
            .into_iter()
            .enumerate()
            .max_by_key(|(pos, count)| (*count, Reverse(*pos)))
            .map(|(pos, _)| pos)
            .unwrap() as i32
    }
}
