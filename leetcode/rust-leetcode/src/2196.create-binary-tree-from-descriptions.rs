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
use std::collections::{HashMap,HashSet};
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> TypeNode {
        let mut map = HashMap::with_capacity(descriptions.len()+1);
        let mut children = HashSet::with_capacity(descriptions.len()+1);
        for d in descriptions.iter() {
            let mut node = map.entry(d[0]).or_insert((-1, -1));
            if d[2] == 1 {
                node.0 = d[1];
            } else {
                node.1 = d[1];
            }
            children.insert(d[1]);
        }
        let mut root = 0;
        for &node in map.keys() {
            if !children.contains(&node) {
                root = node;
                break;
            }
        }
        // println!("{root} {:?}", map);
        fn build_tree(root: i32, map: &HashMap<i32, (i32, i32)>) -> TypeNode {
            match map.get(&root) {
                None => Some(Rc::new(RefCell::new(TreeNode::new(root)))),
                Some(&(l, r)) => {
                    let mut node = TreeNode::new(root);
                    if l != -1 {
                        node.left = build_tree(l, map);
                    }
                    if r != -1 {
                        node.right = build_tree(r, map);
                    }
                    Some(Rc::new(RefCell::new(node)))
                }
            }
        }
        build_tree(root, &map)
    }
}

