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
    pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let list2head = list2;
        let mut list2tail = list2.as_mut().unwrap();
        while list2tail.next.is_some() {
            list2tail = list2tail.next.as_mut().unwrap();
        }
        // println!("{:?}",list2tail);
        let mut dummy = list1.as_mut().unwrap();
        for i in 0..a-1 {
            dummy = dummy.next.as_mut().unwrap();
        }
        let mut binding = dummy.clone();
        let mut dummy2 = binding.next.as_mut().unwrap();
        dummy.next = list2;
        for i in a..b {
            dummy2 = dummy2.next.as_mut().unwrap();
        }
        while dummy.next.is_some() {
            dummy = dummy.next.as_mut().unwrap();
        }
        dummy.next = dummy2.next.clone();
        // println!("{:?}",dummy);
        // println!("{:?}",dummy2);
        list1
    }
}
