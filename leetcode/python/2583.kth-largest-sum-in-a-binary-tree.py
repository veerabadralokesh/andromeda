# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def kthLargestLevelSum(self, root: Optional[TreeNode], k: int) -> int:
        levels = [0]
        q = deque([(root, 0)])
        while q:
            node, level = q.popleft()
            if level == len(levels):
                levels.append(0)
            levels[level] += node.val
            if node.left:
                q.append((node.left, level + 1))
            if node.right:
                q.append((node.right, level + 1))
        levels.sort(reverse=True)
        return -1 if k > len(levels) else levels[k-1]

