/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func modifiedList(nums []int, head *ListNode) *ListNode {
    dummy := ListNode {
        Val: 0,
        Next: head,
    }

    set := make(map[int]bool)
    for _, n := range nums {
        set[n] = true
    }

    curr := &dummy

    for curr.Next != nil {
        if _, ok := set[curr.Next.Val]; ok {
            curr.Next = curr.Next.Next
        } else {
            curr = curr.Next
        }
    }

    return dummy.Next
}

