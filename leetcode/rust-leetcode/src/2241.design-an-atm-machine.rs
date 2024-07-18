struct ATM {
    notes: [u32; 5],
    denominations: [i32; 5]
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {

    fn new() -> Self {
        Self {notes: [0, 0, 0, 0, 0], denominations: [500, 200, 100, 50, 20]}
    }
    
    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, c) in banknotes_count.into_iter().enumerate() {
            self.notes[4-i] += c as u32;
        }
    }
    
    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut i = 0usize;
        let mut ans = vec![0; 5];
        let mut flag = true;
        while amount > 0 && i < 5 {
            if self.notes[i] > 0 && amount >= self.denominations[i] {
                if amount < self.denominations[i] && flag {
                    return vec![-1];
                }
                let note_count = (amount/self.denominations[i]).min(self.notes[i] as i32);
                ans[i] = note_count;
                amount = (amount - note_count * self.denominations[i]);
                flag = false;
            }
            i += 1;
        }
        if amount > 0 {
            return vec![-1];
        }
        for i in 0..5 {
            self.notes[i] -= (ans[i] as u32);
        }
        ans.reverse();
        ans
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */

/* */

pub struct ATM {
    contents: [u32; 5],
}
impl ATM {
    pub fn new() -> Self {
        Self { contents: [0; 5] }
    }
    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        assert!(banknotes_count.len() == 5, "Invalid banknotes count");
        for i in 0..5 {
            self.contents[i] += banknotes_count[i] as u32;
        }
    }
    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        assert!(amount >= 1);
        let mut amount = amount as u32;
        const VALUE: [u32; 5] = [20, 50, 100, 200, 500];
        let mut issued = vec![0i32; 5];
        let mut i: i8 = 4;
        while i >= 0 && amount > 0 {
            let needed = std::cmp::min(amount / VALUE[i as usize], self.contents[i as usize]);
            issued[i as usize] += needed as i32;
            amount -= needed * VALUE[i as usize];
            i -= 1;
        }
        if amount > 0 {
            // Abort transaction
            issued.truncate(1);
            issued[0] = -1;
        } else {
            for i in 0..5 {
                self.contents[i] -= issued[i] as u32;
            }
        }
        issued
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */
