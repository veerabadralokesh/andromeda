# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        def leaves(root: Optional[TreeNode], leafList: List[int]):
            if root is None:
                return
            if root.left is None and root.right is None:
                leafList.append(root.val)
                return
            leaves(root.left, leafList)
            leaves(root.right, leafList)
        
        leaves1 = []
        leaves(root1, leaves1)
        leaves2 = []
        leaves(root2, leaves2)

        return leaves1 == leaves2

