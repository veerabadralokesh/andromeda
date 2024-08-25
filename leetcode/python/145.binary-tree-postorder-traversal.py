# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        def postorder(node: Optional[TreeNode], post: List[int]):
            if node is None:
                return
            postorder(node.left, post)
            postorder(node.right, post)
            post.append(node.val)
        ans = []
        postorder(root, ans)
        return ans

