impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let n = n as usize;
        let mut sums = vec![0; n+1];
        let mut counts = vec![0; 100];
        for i in 1..=9.min(n) {sums[i] = i; counts[i]=1;}
        let mut max_count = 1;
        for i in 10..=n {
            let sum = sums[i/10] + (i % 10);
            counts[sum] += 1;
            sums[i] = sum;
            max_count = max_count.max(counts[sum]);
        }
        counts.into_iter().filter(|&c| c == max_count).count() as _
    }
}

