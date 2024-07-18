// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0i32;
        let mut q = VecDeque::new();
        q.push_back((root.clone(), (1, 1)));
        while let Some((node, (x, y))) = q.pop_front() {
            match (node) {
                None => continue,
                Some(node) => {
                    let node = node.borrow();
                    if y % 2 == 0 {
                        ans += node.val;
                    }
                    q.push_back((node.left.clone(), (node.val, x)));
                    q.push_back((node.right.clone(), (node.val, x)));
                }
            }
        }
        ans
    }
}

/* */

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        let mut ret = 0;
        let root = root.unwrap();

        let mut arr: Vec<(Rc<RefCell<TreeNode>>, u8)> = Vec::new();
        arr.push((root.clone(), 0));
        
        while !arr.is_empty(){
            let mut narr: Vec<(Rc<RefCell<TreeNode>>, u8)> = Vec::new();
            for (node, mask) in arr.iter(){
                let node = node.borrow();
                if (mask & 1) != 0 {
                    ret += node.val;
                }
                let mut nmask = mask >> 1;
                if node.val % 2 == 0 {
                    nmask |= 2;
                }
                if let Some(left) = &node.left{
                    narr.push((left.clone(), nmask));
                }
                if let Some(right) = &node.right{
                    narr.push((right.clone(), nmask));
                }

            }
            arr = narr; 
        }
        
        return ret;
    }
}
