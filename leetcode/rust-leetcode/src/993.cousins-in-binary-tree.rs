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
    pub fn is_cousins(root: TypeNode, x: i32, y: i32) -> bool {
        fn dfs(root: &TypeNode, nodes: &mut Vec<(i32,i32)>, depth: i32, x: i32, y: i32, parent: i32) {
            if nodes.len() == 2 {
                return;
            }
            match root {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if node.val == x || node.val == y {
                        nodes.push((depth, parent));
                    }
                    dfs(&node.left, nodes, depth+1, x, y, node.val);
                    dfs(&node.right, nodes, depth+1, x, y, node.val);
                }
            }
        }
        let mut nodes = vec![];
        dfs(&root, &mut nodes, 0, x, y, -1);
        nodes[0].0 == nodes[1].0 && nodes[0].1 != nodes[1].1
    }
}

