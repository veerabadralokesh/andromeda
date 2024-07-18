impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut plant_grow = plant_time.into_iter().zip(grow_time).collect::<Vec<_>>();
        plant_grow.sort_unstable_by_key(|pg| -pg.1);
        let (mut time, mut ans) = (0, 0);
        for (plant, grow) in plant_grow.into_iter() {
            time += plant;
            ans = ans.max(time + grow);
        }
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut v = plant_time
            .into_iter()
            .zip(grow_time.into_iter())
            .collect::<Vec<_>>();
        v.sort_unstable_by_key(|(_, g)| std::cmp::Reverse(*g));
        v.into_iter()
            .fold((0, 0), |(t, m), (p, g)| (t + p, m.max(t + p + g)))
            .1
    }
}
