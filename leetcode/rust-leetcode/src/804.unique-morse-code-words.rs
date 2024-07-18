use std::collections::HashMap;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut map:HashMap<String, i32> = HashMap::new();
        let morse = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        for word in words.iter() {
            // let test:String = ;
            map.entry(word.bytes().map(|c| morse[(c-b'a') as usize]).collect::<Vec<&str>>().join("")).or_insert(1);
            // println!("{:?}", test);
            // println!("{}", map.len());
        }
        map.len() as i32
    }
}