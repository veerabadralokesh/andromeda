struct BrowserHistory {
    history: Vec<String>,
    current: usize,
    fwd: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![String::from(homepage)],
            current: 0,
            fwd: 0,
        }
    }
    
    fn visit(&mut self, url: String) {
        if self.fwd == 0 {
            // println!("visit 0 {:?}", self.history);
            self.history.push(url);
            self.current += 1;
        } else {
            // println!("visist {} {:?}", self.fwd, self.history);
            while self.fwd > 0 {
                self.history.pop();
                self.fwd -= 1;
            }
            self.history.push(url);
            self.current += 1;
            // println!("visist {} {:?}", self.fwd, self.history);
        }
    }
    
    fn back(&mut self, steps: i32) -> String {
        let temp = self.current;
        self.current -= self.current.min(steps as usize);
        self.fwd += temp - self.current;
        // println!("back {} {} {:?}", self.fwd, self.current, self.history);
        self.history[self.current].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let temp = self.fwd.min(steps as usize);
        self.current += temp;
        self.fwd -= temp;
        // println!("forward {steps} {:?}", self.history);
        self.history[self.current].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
