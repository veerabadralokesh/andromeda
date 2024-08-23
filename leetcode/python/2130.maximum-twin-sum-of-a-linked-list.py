# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def pairSum(self, head: Optional[ListNode]) -> int:
        if head is None:
            return 0
        slow = head
        fast = head
        prev = None
        while fast and fast.next:
            fast = fast.next.next
            temp = slow.next
            slow.next = prev
            prev = slow
            slow = temp
        
        ans = 0
        while slow:
            ans = max(ans, slow.val + prev.val)
            slow = slow.next
            prev = prev.next

        return ans

