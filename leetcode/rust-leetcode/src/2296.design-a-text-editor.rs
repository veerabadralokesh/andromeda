use std::cmp::{min, max};
struct TextEditor {
    left: Vec<char>,
    right: Vec<char>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {

    fn new() -> Self {
        Self {left: vec![], right: vec![]}
    }
    
    fn add_text(&mut self, text: String) {
        for c in text.chars() {
            self.left.push(c);
        }
    }
    
    fn delete_text(&mut self, k: i32) -> i32 {
        let k = min(self.left.len() as i32, k);
        for i in 0..k {
            self.left.pop();
        }
        k
    }
    
    fn cursor_left(&mut self, k: i32) -> String {
        let k = min(self.left.len() as i32, k);
        for i in 0..k {
            self.right.push(self.left.pop().unwrap());
        }
        let mut slice = &self.left[max(self.left.len() as i32 - 10, 0) as usize..];
        slice.iter().collect::<String>()
    }
    
    fn cursor_right(&mut self, k: i32) -> String {
        let k = min(self.right.len() as i32, k);
        for i in 0..k {
            self.left.push(self.right.pop().unwrap());
        }
        self.cursor_left(0)
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */
