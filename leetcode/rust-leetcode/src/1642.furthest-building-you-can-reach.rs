impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut d:i32 = 0;
        let mut lused:Vec<i32> = Vec::with_capacity(ladders as usize);
        let mut bused:i32 = 0;
        let mut i:usize = 1;
        while (i < heights.len()) {
            d = heights[i] - heights[i-1];
            if d <= 0 {
                
            } else if (lused.len() as i32) < ladders {
                lused.push(d);
                if (lused.len() as i32) == ladders {
                    lused.sort();
                }
            } else if ladders > 0 && d > lused[0] {
                bused += lused[0];
                lused[0] = d;
                lused.sort();
            } else {
                bused += d;
            }
            if bused > bricks {
                break;
            }
            i += 1;
        }
        (i-1) as i32
    }
}


use std::collections::BinaryHeap;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        for i in 0..heights.len()-1 {
            if heights[i+1] <= heights[i] {
                continue;
            }
            let diff = heights[i+1] - heights[i];
            bricks -= diff;
            max_heap.push(diff);
            if bricks < 0 {
                bricks += max_heap.pop().unwrap();
                if ladders > 0 {
                    ladders -= 1;
                } else {
                    return i as i32;
                }
            }
        }
        heights.len() as i32 -1
    }
}
