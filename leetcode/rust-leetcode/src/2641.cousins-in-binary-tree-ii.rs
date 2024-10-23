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
    pub fn replace_value_in_tree(root: TypeNode) -> TypeNode {
        fn dfs(node: &TypeNode, depth: usize, levelSums: &mut Vec<i32>) {
            match node {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if depth == levelSums.len() {
                        levelSums.push(0);
                    }
                    levelSums[depth] += node.val;
                    dfs(&node.left, depth+1, levelSums);
                    dfs(&node.right, depth+1, levelSums);
                }
            }
        }

        let mut levelSums = vec![0];
        dfs(&root, 0, &mut levelSums);
        
        fn buildTree(node: &TypeNode, depth: usize, levelSums: &Vec<i32>, siblingSum: i32) -> TypeNode {
            match node {
                None => None,
                Some(node) => {
                    let node = node.borrow();
                    let mut ans = TreeNode::new(levelSums[depth] - siblingSum);
                    let childSum = if node.left.is_some() {
                        node.left.clone().unwrap().borrow().val
                    } else {
                        0
                    } + if node.right.is_some() {
                        node.right.clone().unwrap().borrow().val
                    } else {
                        0
                    };
                    ans.left = buildTree(&node.left, depth+1, levelSums, childSum);
                    ans.right = buildTree(&node.right, depth+1, levelSums, childSum);
                    Some(Rc::new(RefCell::new(ans)))
                }
            }
        }
        buildTree(&root, 0, &levelSums, levelSums[0])
    }
}

