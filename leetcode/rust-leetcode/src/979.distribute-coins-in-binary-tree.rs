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
    pub fn distribute_coins(root: TypeNode) -> i32 {
        fn dfs(root: &TypeNode, moves: &mut i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    let l_moves = dfs(&node.left, moves);
                    let r_moves = dfs(&node.right, moves);
                    *moves += l_moves.abs() + r_moves.abs();
                    ((node.val - 1) + l_moves + r_moves)
                }
            }
        }
        let mut moves = 0i32;
        dfs(&root, &mut moves);
        moves
    }
}
