# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        odd = head
        even = head.next
        evenHead = even
        cur = head
        for _ in range(2):
            if cur is not None:
                cur = cur.next
        odd.next = None
        if even:
            even.next = None
        i = 1
        while cur is not None:
            if i == 1:
                odd.next = cur
                cur = cur.next
                odd = odd.next
                odd.next = None
                i = 0
            else:
                even.next = cur
                cur = cur.next
                even = even.next
                even.next = None
                i = 1
        odd.next = evenHead
        return head

