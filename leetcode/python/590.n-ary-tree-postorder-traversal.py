"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        def dfs(root, arr):
            if root is None:
                return
            for child in root.children:
                dfs(child, arr)
            arr.append(root.val)
        ans = []
        dfs(root, ans)
        return ans

