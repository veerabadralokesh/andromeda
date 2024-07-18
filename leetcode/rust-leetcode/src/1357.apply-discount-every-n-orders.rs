use std::collections::HashMap;
struct Cashier {
    product_prices: HashMap<i32,i32>,
    discount: f64,
    n: i32,
    c: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut product_prices = HashMap::new();
        for i in 0..products.len() {
            product_prices.insert(products[i], prices[i]);
        }
        Cashier {
            product_prices,
            discount:discount as f64,
            n,
            c: 0,
        }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.c += 1;
        let mut total = 0i32;
        for i in 0..product.len() {
            total += (self.product_prices.get(&product[i]).unwrap() * amount[i]);
        }
        let mut bill = total as f64;
        if self.c == self.n {
            self.c = 0;
            bill *= (1.0-(self.discount/100.0));
        }
        bill
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
