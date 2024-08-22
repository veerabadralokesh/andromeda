class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        s = 0

        for i in range(k):
            s += nums[i]
        
        maxSum = s

        for i in range(k, len(nums)):
            s = s + nums[i] - nums[i-k]
            maxSum = max(s, maxSum)
        
        return maxSum / k

