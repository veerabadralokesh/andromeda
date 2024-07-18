struct StockSpanner {
    stock: Vec<i32>,
    span: Vec<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {stock: vec![], span: vec![]}
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        
        if !self.stock.is_empty() {
            let mut lookup = self.stock.len() - 1;
            while self.stock[lookup] <= price {
                span += self.span[lookup];
                if lookup < self.span[lookup] {
                    break;
                }
                lookup -= self.span[lookup];
            }
        }
        self.stock.push(price);
        self.span.push(span);

        span as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

 