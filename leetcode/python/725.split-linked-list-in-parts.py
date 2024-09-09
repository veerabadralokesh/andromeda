# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def splitListToParts(self, head: Optional[ListNode], k: int) -> List[Optional[ListNode]]:
        temp = head
        total = 0
        while temp:
            temp = temp.next
            total += 1

        ans = []

        temp = head

        perSplit = total // k
        extra = total % k
        # print(perSplit, extra)
        count = 0
        i = 0
        while temp:
            ans.append(temp)
            count = 1
            while temp and count < perSplit:
                temp = temp.next
                count += 1
            if i < extra and count < perSplit + 1 and temp:
                temp = temp.next
            i += 1
            init = 0
            if temp:
                temp2 = temp.next
                temp.next = None
                temp = temp2
            
        for _ in range(k - len(ans)):
            ans.append(None)
        
        return ans

