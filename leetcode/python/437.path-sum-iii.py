# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        count = 0
        def dfs(node: Optional[TreeNode], path: List[int]):
            if node is None:
                return
            path.append(node.val)
            cumSum = 0
            for nval in path[::-1]:
                cumSum += nval
                if cumSum == targetSum:
                    nonlocal count
                    count += 1
            dfs(node.left, path)
            dfs(node.right, path)
            path.pop()
        
        dfs(root, [])
        return count

""" """

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        
        d = collections.defaultdict(int)
        d[0] = 1

        def dfs(node: Optional[TreeNode], total: int):
            if node is None:
                return 0
            
            total += node.val
            count = d[total - targetSum]

            d[total] += 1

            count += (dfs(node.left, total) + dfs(node.right, total))

            d[total] -= 1

            return count
        
        
        return dfs(root, 0)

