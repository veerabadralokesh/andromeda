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
    pub fn generate_trees(n: i32) -> Vec<TypeNode> {
        fn generate_tree(nums: &[i32], n: usize) -> Vec<TypeNode> {
            if nums.len() == 0 {
                return vec![None]
            }
            if nums.len() == 1 {
                return vec![Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))))]
            }
            let l = nums.len();
            // https://en.wikipedia.org/wiki/Catalan_number
            let cap = (2..=l).map(|i| l + i).fold(1, |acc, x| acc * x)/(2..=l).fold(1, |acc, x| acc * x);
            let mut ans = Vec::with_capacity(cap);
            for i in 0..l {
                let left_trees = generate_tree(&nums[..i], n);
                let right_trees = generate_tree(&nums[i+1..], n);
                for j in 0..left_trees.len() {
                    for k in 0..right_trees.len() {
                        let mut tnode = TreeNode::new(nums[i]);
                        tnode.left = left_trees[j].clone();
                        tnode.right = right_trees[k].clone();
                        ans.push(Some(Rc::new(RefCell::new(tnode))));
                    }
                }
            }
            // if nums.len() == n {
            //     println!("{} {}", ans.len(), (cap));
            // }
            ans
        }
        let nums = (1..=n).collect::<Vec<_>>();
        generate_tree(&nums, n as usize)
    }
}

