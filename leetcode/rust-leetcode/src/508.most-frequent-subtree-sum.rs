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
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn find_frequent_tree_sum(root: TypeNode) -> Vec<i32> {
        let mut map = HashMap::new();
        fn dfs(root: &TypeNode, map: &mut HashMap<i32,i32>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    let subtree_sum = node.val + dfs(&node.left, map) + dfs(&node.right, map);
                    *map.entry(subtree_sum).or_insert(0) += 1;
                    subtree_sum
                }
            }
        }
        dfs(&root, &mut map);
        let mut subtree_sums = map.keys().cloned().collect::<Vec<i32>>();
        subtree_sums.sort_by_key(|k| -map.get(k).unwrap());
        let count = map.get(&subtree_sums[0]).unwrap();
        subtree_sums.into_iter().filter(|ss| map.get(&ss).unwrap() == count).collect()
    }
}

