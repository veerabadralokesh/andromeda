
impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        // let mut start = products.binary_search(&search_word).unwrap();
        let mut words = vec![Vec::new(); search_word.len()];
        // let mut iwords = Vec::new();
        let mut prodb = products.clone().into_iter().map(|s| s.clone().into_bytes()).collect::<Vec<Vec<u8>>>();
        // println!("{:?}",prodb);
        let search_word = search_word.into_bytes();
        let mut start = 0usize;
        let mut end = prodb.len();
        // let mut idx = start;
        // println!("{:?} {start} {end}",search_word);
        for (i,b) in search_word.into_iter().enumerate() {
            for idx in start..end {
                if i < prodb[idx].len() {
                    if b == prodb[idx][i] {
                        if words[i].len() == 0 {
                            start = idx;
                        }
                        if words[i].len() < 3 {
                            words[i].push(products[idx].clone());
                        }
                    } else if words[i].len() != 0  {
                        end = idx;
                        break;
                    }
                }
            }
            if words[i].len() == 0 {
                break;
            }
            // println!("{start}, {end}");
        }
        words
    }
}
