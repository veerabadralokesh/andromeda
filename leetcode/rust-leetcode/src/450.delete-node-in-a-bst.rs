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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> TypeNode {
        if nums.len() == 0 {return None;}
        fn build_bst(nums: &Vec<i32>, l: usize, r: usize) -> TypeNode {
            if l > r {
                return None;
            }
            let m = (l + r)/2;
            let mut node = TreeNode::new(nums[m]);
            if m > 0 {
                node.left = build_bst(nums, l, m-1);
            }
            node.right = build_bst(nums, m+1, r);
            Some(Rc::new(RefCell::new(node)))
        }
        build_bst(&nums, 0, nums.len()-1)
    }
    pub fn delete_node(root: TypeNode, key: i32) -> TypeNode {
        if root.is_none() {
            return root;
        }
        fn traverse(root: &TypeNode, vec: &mut Vec<i32>, key: i32) {
            match (root) {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    if key != node.val {
                        vec.push(node.val);
                    }
                    traverse(&node.left, vec, key);
                    traverse(&node.right, vec, key);
                }
            }
        }
        let mut sorted_array = Vec::new();
        traverse(&root, &mut sorted_array, key);
        sorted_array.sort();
        Self::sorted_array_to_bst(sorted_array)
    }
}
