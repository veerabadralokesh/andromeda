use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senate = senate.chars().collect::<VecDeque<char>>();
        // println!("{:?}", senate);
        let mut rcount = senate.iter().filter(|&c| *c=='R').count();
        let mut dcount = senate.iter().filter(|&c| *c=='D').count();
        // println!("{rcount}, {dcount}");
        let mut rr = 0;
        let mut rd = 0;
        while rcount > 0 && dcount > 0 {
            match (senate[0]) {
                'R' => {
                    if rr > 0 {
                        rr -=1;
                        rcount -= 1;
                        senate.pop_front();
                    } else {
                        senate.rotate_left(1);
                        rd += 1;
                    }
                },
                _ => {
                    if rd > 0 {
                        rd -= 1;
                        dcount -= 1;
                        senate.pop_front();
                    } else {
                        senate.rotate_left(1);
                        rr += 1;
                    }
                }
            }
        }

        if rcount > 0 {
            String::from("Radiant")
        } else {
            String::from("Dire")
        }
    }
}
