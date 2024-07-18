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
    pub fn balance_bst(root: TypeNode) -> TypeNode {
        fn inorder(root: &TypeNode, nums: &mut Vec<i32>) {
            if root.is_some() {
                let node = root.as_ref().unwrap().borrow();
                inorder(&node.left, nums);
                nums.push(node.val);
                inorder(&node.right, nums);
            }
        }

        fn build(nums: &[i32]) -> TypeNode {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len()/2;
            let mut tnode = TreeNode::new(nums[mid]);
            tnode.left = build(&nums[..mid]);
            tnode.right = build(&nums[mid+1..]);
            Some(Rc::new(RefCell::new(tnode)))
        }

        let mut nums = Vec::new();
        inorder(&root, &mut nums);
        build(&nums)
    }
}

