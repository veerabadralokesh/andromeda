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
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {return None;}
        let mut map = HashMap::new();
        for (i, &n) in inorder.iter().enumerate() {
            map.insert(n, i);
        }
        fn build(preorder: &Vec<i32>, map: &HashMap<i32,usize>, start: usize, end: usize, idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            if start > end {
                return None;
            }
            let val = preorder[*idx];
            let inorder_idx = *map.get(&val).unwrap();
            *idx += 1;
            let mut tnode = TreeNode::new(val);
            if start < inorder_idx {
                tnode.left = build(preorder, map, start, inorder_idx - 1, idx);
            }
            tnode.right = build(preorder, map, inorder_idx + 1, end, idx);
            Some(Rc::new(RefCell::new(tnode)))
        }
        let mut idx = 0;
        build(&preorder, &map, 0, inorder.len()-1, &mut idx)
    }
}

