impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let mut pigs = 0;
        let mut buckets_covered = 1;
        let mut pig_to_buckets = minutes_to_test / minutes_to_die + 1;
        while buckets_covered < buckets {
            pigs += 1;
            buckets_covered *= pig_to_buckets;
        }
        pigs
    }
}

