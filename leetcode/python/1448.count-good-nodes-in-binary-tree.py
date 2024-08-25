# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        count = 0
        def countGood(root: TreeNode, maxNode: int):
            if root is None:
                return
            if root.val >= maxNode:
                nonlocal count
                count += 1
            countGood(root.left, max(maxNode, root.val))
            countGood(root.right, max(maxNode, root.val))
        
        countGood(root, -float('inf'))
        return count

