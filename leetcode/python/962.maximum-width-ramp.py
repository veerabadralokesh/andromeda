class Solution:
    def maxWidthRamp(self, nums: List[int]) -> int:
        stack = []
        ans = 0
        for i, n in enumerate(nums):
            if not stack or nums[stack[-1]] >= n:
                stack.append(i)
        
        for i, n in reversed(list(enumerate(nums))):
            while stack and n >= nums[stack[-1]]:
                ans = max(ans, i - stack.pop())

        return ans

