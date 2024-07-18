impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {return 0;}
        let amount = amount as usize;
        let mut counts = vec![i32::MAX; amount + 1];
        counts[0] = 0;
        let coins = coins.iter().map(|&c| c as usize).collect::<Vec<_>>();
        for a in 1..counts.len() {
            for &coin in coins.iter() {
                if coin > a {
                    continue;
                }
                if counts[a-coin] != i32::MAX {
                    counts[a] = counts[a].min(counts[a-coin] + 1);
                }
            }
        }
        match counts[amount] {
            i32::MAX => -1,
            c @ _ => c
        }
    }
}

