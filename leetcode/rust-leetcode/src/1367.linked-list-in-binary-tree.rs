// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: TypeNode) -> bool {
        let mut head = head;
        let mut list = Vec::new();
        while let Some(node) = head {
            list.push(node.val);
            head = node.next;
        }
        fn dfs(root: &TypeNode, list: &Vec<i32>, ans: &mut bool, path: &mut Vec<i32>) {
            if *ans {
                return;
            }
            match root {
                None => {
                    if path.len() >= list.len() {
                        let mut flag = true;
                        for i in 0..path.len()-list.len()+1 {
                            if path[i] == list[0] {
                                flag = true;
                                for j in 0..list.len() {
                                    if list[j] != path[i+j] {
                                        flag = false;
                                        break;
                                    }
                                }
                                if flag {
                                    *ans = true;
                                    return;
                                }
                            }
                        }
                    }
                },
                Some(node) => {
                    let node = node.borrow();
                    path.push(node.val);
                    let ldepth = dfs(&node.left, list, ans, path);
                    let rdepth = dfs(&node.right, list, ans, path);
                    path.pop();
                }
            }
        }
        let mut ans = false;
        let mut path = vec![];
        dfs(&root, &list, &mut ans, &mut path);
        ans
    }
}

