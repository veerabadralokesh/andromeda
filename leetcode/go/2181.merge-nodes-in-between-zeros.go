/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeNodes(head *ListNode) *ListNode {
    dummy := head
    temp := head.Next
    for temp != nil {
        if temp.Val == 0 && temp.Next != nil {
            dummy = dummy.Next
            dummy.Val = 0
        } else {
            dummy.Val += temp.Val
        }
        temp = temp.Next
    }
    dummy.Next = nil
    return head
}

