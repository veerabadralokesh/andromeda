/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseKGroup(head *ListNode, k int) *ListNode {
    if head == nil {return head}
    count := 0
    tail := head
    temp := head
    for count < k {
        if tail == nil {
            return head
        }
        temp = tail
        tail = tail.Next
        count++
    }
    temp.Next = nil
    // printList(head)
    // printList(tail)
    new_head := reverseKGroup(tail, k)
    // printList(new_head)
    for count > 0 {
        node := head
        head = node.Next
        node.Next = new_head
        new_head = node
        count--
    }
    return new_head
}

// func printList(head *ListNode) {
//     list := []int{}
//     tail := head
//     for tail != nil {
//         list = append(list, tail.Val)
//         tail = tail.Next
//     }
//     fmt.Println(list)
// }


