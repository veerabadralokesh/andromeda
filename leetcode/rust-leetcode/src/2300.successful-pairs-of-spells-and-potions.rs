impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();
        let potions = potions.iter().map(|&p| p as i64).collect::<Vec<_>>();
        let mut ans = vec![0; spells.len()];
        for (i, &spell) in spells.iter().enumerate() {
            let spell = spell as i64;
            let mut left = 0;
            if spell * potions[left] >= success {
                ans[i] = (potions.len()) as i32;
                continue;
            }
            let mut right = potions.len()-1;
            if spell * potions[right] < success {
                continue;
            }
            while left <= right {
                let mid = (left + right)/2;
                // println!("{spell} {mid} {left} {right}");
                if spell * potions[mid] < success {
                    if mid < potions.len() - 1 && spell * potions[mid+1] >= success {
                        ans[i] = (potions.len() - mid - 1) as i32;
                        break;
                    } else {
                        left = mid + 1;
                    }
                } else {
                    right = mid;
                }
            }
        }
        ans
    }
}
