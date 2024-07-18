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
    pub fn recover_from_preorder(traversal: String) -> TypeNode {
        let tb = traversal.into_bytes();
        let mut preorder = vec![];
        let (mut depth, mut node_val, mut idx, mut is_next_node) = (0, 0, 0, true);
        for &b in tb.iter() {
            match b {
                b'-' => {
                    if !is_next_node {
                        preorder.push((depth, node_val));
                        node_val = 0;
                        depth = 0;
                        is_next_node = true;
                    }
                    depth += 1;
                },
                x => {
                    is_next_node = false;
                    let n = (b - b'0') as i32;
                    node_val = node_val * 10 + n;
                }
            }
        }
        preorder.push((depth, node_val));

        fn build(p: &[(i32, i32)]) -> TypeNode {
            if p.len() == 0 {
                return None;
            }
            if p.len() == 1 {
                return Some(Rc::new(RefCell::new(TreeNode::new(p[0].1))));
            }
            let mut tnode = TreeNode::new(p[0].1);
            let mut i = 2;
            while i < p.len() {
                if p[i].0 == p[1].0 {
                    break;
                }
                i += 1;
            }
            tnode.left = build(&p[1..i]);
            if i < p.len() {
                tnode.right = build(&p[i..]);
            }
            Some(Rc::new(RefCell::new(tnode)))
        }

        build(&preorder)
    }
}

