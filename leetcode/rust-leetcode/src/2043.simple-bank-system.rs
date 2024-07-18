struct Bank {
    balance: Vec<i64>,
    n: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        let n = balance.len() + 1;
        let mut b = Vec::with_capacity(n);
        b.push(0);
        for i in balance.into_iter() {
            b.push(i);
        }
        Self {
            balance: b,
            n: n as i32,
        }
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        self.is_valid_acc(account1) && self.is_valid_acc(account2) && match self.balance[account1 as usize] {
            b if b >= money => {
                self.balance[account1 as usize] -= money;
                self.balance[account2 as usize] += money;
                true
            },
            _ => false
        }
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        self.is_valid_acc(account) && {
            self.balance[account as usize] += money;
            true
        }
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        self.is_valid_acc(account) && match self.balance[account as usize] {
            b if b >= money => {
                self.balance[account as usize] -= money;
                true
            },
            _ => false
        }
    }

    fn is_valid_acc(&self, account: i32) -> bool {
        account > 0 && account <= self.n
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
