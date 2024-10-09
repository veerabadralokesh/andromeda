class Solution:
    def minIncrementOperations(self, nums: List[int], k: int) -> int:
        p1, p2, p3 = 0, 0, 0
        for n in nums:
            dp = min(p1, min(p2, p3)) + max(0, k - n)
            p3 = p2
            p2 = p1
            p1 = dp
        return min(p1, min(p2, p3))

