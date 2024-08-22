class Solution:
    def pivotIndex(self, nums: List[int]) -> int:
        s = sum(nums)
        cs = 0
        for i in range(len(nums)):
            if cs * 2 + nums[i] == s:
                return i
            cs += nums[i]
        return -1

