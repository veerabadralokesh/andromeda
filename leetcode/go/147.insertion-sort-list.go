/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func insertionSortList(head *ListNode) *ListNode {
    if head == nil {
        return nil
    }
    unsorted := head
    sorted := &ListNode {
        Val: 0,
        Next: nil,
    }
    for unsorted != nil {
        nodeToInsert := unsorted
        unsorted = unsorted.Next
        nodeToInsert.Next = nil
        sortedRef := sorted
        for sortedRef.Next != nil && sortedRef.Next.Val < nodeToInsert.Val {
            sortedRef = sortedRef.Next
        }
        nodeToInsert.Next = sortedRef.Next
        sortedRef.Next = nodeToInsert
    }
    return sorted.Next
}

