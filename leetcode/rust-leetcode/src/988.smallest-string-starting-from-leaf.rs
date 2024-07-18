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
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn smallest_from_leaf(root: TypeNode) -> String {
        let mut s:Vec<String> = Vec::new();
        fn dfs(root: &TypeNode, s: &mut Vec<String>, ps: &mut String) {
            match (root) {
                None => {
                    // s.push(ps.chars().rev().collect());
                },
                Some(node) => {
                    let node = node.borrow();
                    let c = ((b'a' as u8) + (node.val as u8)) as char;
                    ps.push(c);
                    if node.left.is_none() && node.right.is_none() {
                        s.push(ps.chars().rev().collect());
                    } else {
                        if node.left.is_some() {
                            dfs(&node.left, s, ps);
                        }
                        if node.right.is_some() {
                            dfs(&node.right, s, ps);
                        }
                    }
                    ps.pop();
                }
            }
        }
        let mut ps = String::new();
        dfs(&root, &mut s, &mut ps);
        // println!("{:?}", s);
        s.sort();
        // println!("{:?}", s);
        s[0].clone()
    }
}
