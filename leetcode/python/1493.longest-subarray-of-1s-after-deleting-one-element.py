class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        l, r, flips, ans = 0, 0, 0, 0
        
        while r < len(nums):
            if nums[r] == 0:
                flips += 1
            while flips > 1:
                if nums[l] == 0:
                    flips -= 1
                l += 1
            ans = max(ans, r - l)
            r += 1
        return ans

