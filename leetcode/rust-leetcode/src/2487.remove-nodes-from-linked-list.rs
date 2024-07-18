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
impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nums = vec![];
        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }
        let mut ans = vec![];
        ans.push(nums[0]);
        for i in 1..nums.len() {
            while ans.len() > 0 && *ans.last().unwrap() < nums[i] {
                ans.pop();
            }
            ans.push(nums[i]);
        }
        let mut head = None;
        for i in (0..ans.len()).rev() {
            let mut lnode = ListNode::new(ans[i]);
            lnode.next = head;
            head = Some(Box::new(lnode));
        }
        head
    }
}

/* */

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
impl Solution {
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed: Option<Box<ListNode>> = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = reversed;
            reversed = Some(node)
        }
        reversed
    }

    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = Self::reverse_list(head);
        let mut cur = reversed.as_mut().unwrap();
        while cur.next.is_some() {
            if cur.val > cur.next.as_ref().unwrap().val {
                cur.next = cur.next.as_mut().unwrap().next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }

        Self::reverse_list(reversed)
    }
}
