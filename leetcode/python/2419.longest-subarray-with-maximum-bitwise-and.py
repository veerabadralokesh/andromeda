class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        maxn = max(nums)
        maxLen = 1
        count = 0
        for n in nums:
            if n == maxn:
                count += 1
            else:
                count = 0
            maxLen = max(count, maxLen)
        return maxLen

