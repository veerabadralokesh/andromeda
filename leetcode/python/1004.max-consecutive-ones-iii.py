class Solution:
    def longestOnes(self, nums: List[int], k: int) -> int:
        l, r, flips, ans = 0, 0, 0, 0
        
        while r < len(nums):
            if nums[r] == 0:
                flips += 1
            while flips > k:
                if nums[l] == 0:
                    flips -= 1
                l += 1
            ans = max(ans, r - l + 1)
            r += 1
        return ans

