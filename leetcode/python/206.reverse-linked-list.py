# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        rhead = head
        prev = None
        while head and head.next is not None:
            rhead = head.next
            head.next = prev
            prev = head
            head = rhead
        head.next = prev
        return head

