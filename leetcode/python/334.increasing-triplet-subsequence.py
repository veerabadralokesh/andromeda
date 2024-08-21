class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:
        i = float('inf')
        j = float('inf')
        for n in nums:
            if i >= n:
                i = n
            elif j >= n:
                j = n
            else:
                return True
        return False

