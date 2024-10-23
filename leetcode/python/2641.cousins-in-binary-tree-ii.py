# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def replaceValueInTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root:
            return root
        
        levelSums = [0]
        def dfs(node:Optional[TreeNode], depth):
            if node is None:
                return
            if depth == len(levelSums):
                levelSums.append(0)
            
            levelSums[depth] += node.val

            dfs(node.left, depth+1)
            dfs(node.right, depth+1)
        
        dfs(root, 0)
        
        def buildTree(root: Optional[TreeNode], depth:int, siblingSum:int) -> Optional[TreeNode]:
            if not root:
                return None
            ans = TreeNode(val=levelSums[depth]-siblingSum)
            childSum = (0 if root.left is None else root.left.val) + (0 if root.right is None else root.right.val)
            if root.left:
                ans.left = buildTree(root.left, depth+1, childSum)
            if root.right:
                ans.right = buildTree(root.right, depth+1, childSum)
            
            return ans
        
        return buildTree(root, 0, root.val)

