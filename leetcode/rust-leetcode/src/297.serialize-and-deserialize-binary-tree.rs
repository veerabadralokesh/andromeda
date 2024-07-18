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
struct Codec {
	
}
type TypeNode = Option<Rc<RefCell<TreeNode>>>;
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: TypeNode) -> String {
        let mut serialized = Vec::new();
        fn dfs(root: &TypeNode, serialized: &mut Vec<String>) {
            match root {
                None => {
                    serialized.push(String::from("null"));
                },
                Some(node) => {
                    let node = node.borrow();
                    serialized.push(node.val.to_string());
                    dfs(&node.left, serialized);
                    dfs(&node.right, serialized);
                }
            }
        }
        dfs(&root, &mut serialized);
        serialized.join(",")
    }
	
    fn deserialize(&self, data: String) -> TypeNode {
        let nums = data.split(",").collect::<Vec<_>>();
        fn preorder(nums: &Vec<&str>, idx: &mut usize) -> TypeNode {
            if nums[*idx] == "null" {
                *idx += 1;
                return None;
            }
            let mut tnode = TreeNode::new(nums[*idx].parse::<i32>().unwrap());
            *idx += 1;
            tnode.left = preorder(nums, idx);
            tnode.right = preorder(nums, idx);
            Some(Rc::new(RefCell::new(tnode)))
        }
        let mut idx = 0;
        preorder(&nums, &mut idx)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

/* */

// LEARN


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
use std::collections::VecDeque;

macro_rules! string_to_btree {
($treestring:expr) => {
{
    println!("Converting string to btree");
    let mut tree: String = $treestring;
    // Check that the string starts and ends with square brackets.
    assert!(tree.starts_with("["));
    assert!(tree.ends_with("]"));
    // Trim the brackets.
    tree.remove(0);
    tree.remove(tree.len()-1);
    // Collect tree into vec of string slices.
    let mut nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = tree.split(",")
        .map(|v|
            match v {
                "null" => None,
                x => Some(Rc::new(RefCell::new(TreeNode::new(x.parse::<i32>().ok()?)))),
            }
        )
        .collect();

    let mut queue = VecDeque::new();
    let root = nodes.pop_front().unwrap();
    queue.push_back(root.clone());

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_back().unwrap() {
                let mut node = node.borrow_mut();
                if let Some(left) = nodes.pop_front() {
                    node.left = left;
                    queue.push_front(node.left.clone());
                }
                if let Some(right) = nodes.pop_front() {
                    node.right = right;
                    queue.push_front(node.right.clone());
                }
            }
        }
    }

    root
}
};
}

macro_rules! btree_to_string {
($treenode:expr) => {
{
    println!("Converting btree to string");
    let head: Option<Rc<RefCell<TreeNode>>> = $treenode;
    let mut v = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(head);
    while let Some(nodeopt) = queue.pop_front() {
        if let Some(node) = nodeopt {
            let mut node = node.borrow_mut();
            let mut val: String = node.val.to_string();
            v.push(val);
            queue.push_back(node.left.take());
            queue.push_back(node.right.take());
        } else {
            v.push("null".to_owned());
        }
    }
    v = v.into_iter().rev().skip_while(|x| x == "null").collect();
    v.reverse();
    let mut out: String = v.join(",");
    out.insert_str(0, "[");
    out.push_str("]");
    println!("{}", out);
    out
}
};
}


struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        btree_to_string!(root)
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        string_to_btree!(data)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */


