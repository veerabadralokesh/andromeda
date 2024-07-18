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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        let mut counts = [0i32; 22000];
        let mut dummy = ListNode::new(-1);
        let mut dummy = Box::new(dummy);
        let mut head = dummy.as_mut();
        for i in 0..lists.len() {
            head.next = lists[i].clone();
            while head.next.is_some() {
                head = head.next.as_mut().unwrap();
                counts[(head.val as usize) + 10000] += 1;
            }
        }
        let total = counts.iter().sum::<i32>();
        if total < 1 {
            return None;
        }
        let mut head = dummy.next.as_mut().unwrap();
        let mut uniq = Vec::new();
        for i in 0..counts.len() {
            if counts[i] > 0 {
                uniq.push((i as i32) - 10000);
            }
        }
        for n in uniq.to_vec() {
            let ni = (10000 + n) as usize;
            while counts[ni] > 0 {
                head.val = n;
                if head.next.is_some() {
                    head = head.next.as_mut().unwrap();
                }
                counts[ni] -= 1;
            }
        }
        Some(dummy.next.unwrap())
    }
}
