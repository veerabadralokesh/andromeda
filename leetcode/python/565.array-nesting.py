class Solution:
    def arrayNesting(self, nums: List[int]) -> int:
        depths = {}
        n = len(nums)
        
        def dfs(i: int, s: set):
            if i in depths:
                return depths[i]
            if i in s:
                return 0
            s.add(i)
            d = 1 + dfs(nums[i], s)
            depths[i] = d
            return d
        
        ans = 0
        for k in range(n):
            ans = max(ans, dfs(k, set()))
        return ans

""" """

class Solution:
    def arrayNesting(self, nums: List[int]) -> int:
        n = len(nums)
        visited = [False] * n

        ans = 0
        for k in range(n):
            if not visited[k]:
                setLength = 0
                j = k

                while not visited[j]:
                    visited[j] = True
                    j = nums[j]
                    setLength += 1
                    ans = max(ans, setLength)
        return ans


