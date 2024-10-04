impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let total = skill.iter().sum::<i32>();
        let n = skill.len() as i32/2;
        if total % n != 0 {
            return -1;
        }
        let teamSkill = total / n;
        let mut rems = vec![0; teamSkill as usize + 1];
        for s in skill {
            if s >= teamSkill {
                return -1;
            }
            rems[s as usize] += 1;
        }
        let mut ans = 0;
        let teamSkill = teamSkill as usize;
        for i in 1..=teamSkill/2 {
            if rems[i] != rems[teamSkill - i] {
                return -1;
            }
            if i != (teamSkill - i) {
                ans += i as i64 * (teamSkill - i) as i64 * rems[i] as i64;
            } else {
                if rems[i] & 1 == 1 {return -1;}
                ans += i as i64 * (teamSkill - i) as i64 * rems[i] as i64 / 2;
            }
        }
        ans
    }
}

